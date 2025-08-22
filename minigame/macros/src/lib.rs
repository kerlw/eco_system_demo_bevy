use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Attribute, Data, DeriveInput, Expr, Fields, Index, Meta, parse_macro_input, spanned::Spanned,
};

/// 自动实现 `Percentage` trait 的派生宏
///
/// 本宏用于为包含数值字段的结构体自动生成百分比计算逻辑，支持两种使用模式：
/// 1. **元组结构体**：通过结构体级属性 `#[percentage(max = ...)]` 指定最大值
/// 2. **具名字段结构体**：通过字段级属性 `#[percentage]` 标记目标字段
///
/// # 设计特性
/// - **编译时检查**：确保 `max > 0`（触发编译错误而非运行时 panic）
/// - **灵活类型**：原始字段支持任意实现 `Into<f32>` 的数值类型
/// - **优先级规则**：字段级属性 > 结构体级属性 > 默认值（100.0）
///
/// # 使用示例
///
/// ## 场景1：元组结构体 + 结构体属性
/// ```rust
/// #[derive(Percentage)]
/// #[percentage(max = 255.0)] // 结构体级属性
/// struct ColorValue(u8);     // 自动使用 .0 字段
///
/// let color = ColorValue(200);
/// assert!((color.value() - 0.784).abs() < 0.001); // 200/255.0 ≈ 0.784
/// ```
///
/// ## 场景2：具名字段 + 字段属性
/// ```rust
/// #[derive(Percentage)]
/// struct Player {
///     health: u32,
///     #[percentage(max = 100.0)] // 字段级属性（显式 max）
///     score: f32,                // 使用此字段计算
/// }
///
/// let player = Player { health: 80, score: 75.0 };
/// assert_eq!(player.value(), 0.75); // 75.0/100.0
/// ```
///
/// ## 场景3：混合优先级（字段级优先）
/// ```rust
/// #[derive(Percentage)]
/// #[percentage(max = 200.0)] // 结构体级属性（备用）
/// struct GameData {
///     #[percentage] // 字段级属性（无显式 max，使用默认 100.0）
///     progress: u8, // 优先级高于结构体属性
/// }
///
/// let data = GameData { progress: 75 };
/// assert_eq!(data.value(), 0.75); // 75/100.0
/// ```
///
/// ## 场景4：泛型支持（默认 max）
/// ```rust
/// #[derive(Percentage)]
/// struct Normalized<T: Into<f32>>(T); // 使用默认 max=100.0
///
/// let norm = Normalized(75.0_f32);
/// assert_eq!(norm.value(), 0.75);
/// ```
///
/// # 属性语法
/// | **属性位置**       | **语法**                      | **作用域**         |
/// |--------------------|-------------------------------|-------------------|
/// | 结构体级           | `#[percentage(max = 表达式)]` | 仅限元组结构体     |
/// | 字段级             | `#[percentage]`               | 具名字段           |
/// | 字段级（带 max）   | `#[percentage(max = 表达式)]` | 具名字段           |
///
/// # 错误处理
/// - ❌ **编译错误**：`max` 表达式结果 ≤ 0
///   ```rust
///   #[derive(Percentage)]
///   #[percentage(max = -5.0)] // 触发编译错误
///   struct InvalidValue(f32);
///   ```
/// - ❌ **字段冲突**：多个字段标记 `#[percentage]`
/// - ❌ **类型错误**：原始字段未实现 `Into<f32>`
///
/// > 通过 `cargo doc --open` 可生成交互式文档，点击示例代码右上角 "Run" 按钮可实时测试
#[proc_macro_derive(Percentage, attributes(percentage))]
pub fn percentage_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;

    // 1. 解析目标字段和最大值表达式
    let (field_expr, max_expr) = match parse_target_field(&input) {
        Ok(res) => res,
        Err(e) => return e.to_compile_error().into(),
    };

    // 2. 生成编译时静态断言（确保 max > 0）
    let max_assert = generate_max_assertion(&max_expr);

    // 3. 生成 trait 实现
    let expanded = quote! {
        #max_assert

        impl Percentage for #struct_name {
            fn value(&self) -> f32 {
                let raw_f32: f32 = #field_expr.into();
                let max_f32: f32 = #max_expr.into();
                raw_f32 / max_f32
            }
        }
    };

    TokenStream::from(expanded)
}

/// 智能定位目标字段和最大值
fn parse_target_field(input: &DeriveInput) -> Result<(proc_macro2::TokenStream, Expr), syn::Error> {
    // 解析结构体级别的 #[percentage(max = ...)]
    let struct_max = parse_struct_max(&input.attrs)?;

    match &input.data {
        // 场景1：元组结构体 + 结构体属性
        Data::Struct(s) if struct_max.is_some() => {
            if let Fields::Unnamed(fields) = &s.fields {
                // 显式检查仅含一个字段
                if fields.unnamed.len() != 1 {
                    return Err(syn::Error::new(
                        input.span(),
                        "结构体级#[percentage]仅支持单字段元组结构体",
                    ));
                }
                let field_index = Index::from(0);
                return Ok((quote!(self.#field_index), struct_max.unwrap()));
            }
            // 非元组结构体报错
            Err(syn::Error::new(
                input.span(),
                "结构体级#[percentage]只能用于元组结构体（如 `struct Name(f32)`）",
            ))
        }

        // 场景2：具名字段 + 字段属性
        Data::Struct(s) => {
            let mut percentage_fields = s
                .fields
                .iter()
                .enumerate()
                .filter_map(|(i, field)| {
                    field
                        .attrs
                        .iter()
                        .find(|attr| attr.path().is_ident("percentage"))
                        .map(|attr| (i, field, attr))
                })
                .collect::<Vec<_>>();

            match percentage_fields.len() {
                0 => Err(syn::Error::new(
                    input.span(),
                    "需在结构体或字段添加#[percentage]属性",
                )),
                1 => {
                    let (i, field, attr) = percentage_fields.pop().unwrap();
                    let max_expr = parse_field_max(attr)?.or(struct_max).unwrap_or_else(|| {
                        syn::parse_str::<Expr>("100.0_f32").expect("默认值解析失败")
                    });

                    // 根据字段类型生成访问表达式
                    let field_expr = if let Some(ident) = &field.ident {
                        quote!(self.#ident)
                    } else {
                        let index = Index::from(i);
                        quote!(self.#index)
                    };
                    Ok((field_expr, max_expr))
                }
                _ => Err(syn::Error::new(
                    input.span(),
                    "仅允许一个字段标记#[percentage]",
                )),
            }
        }
        _ => Err(syn::Error::new(input.span(), "仅支持结构体类型")),
    }
}

/// 解析结构体级别的 max 属性
fn parse_struct_max(attrs: &[Attribute]) -> Result<Option<Expr>, syn::Error> {
    for attr in attrs {
        if attr.path().is_ident("percentage") {
            if let Meta::NameValue(nv) = &attr.meta {
                if nv.path.is_ident("max") {
                    return Ok(Some(nv.value.clone()));
                }
            }
        }
    }
    Ok(None)
}

/// 解析字段级别的 max 属性
fn parse_field_max(attr: &Attribute) -> Result<Option<Expr>, syn::Error> {
    if let Meta::NameValue(nv) = &attr.meta {
        if nv.path.is_ident("max") {
            return Ok(Some(nv.value.clone()));
        }
    }
    Ok(None)
}

/// 生成编译时静态断言（max > 0）
fn generate_max_assertion(max_expr: &Expr) -> proc_macro2::TokenStream {
    quote! {
        const _: () = {
            const MAX: f32 = #max_expr.into();
            static_assertions::const_assert!(MAX > 0.0);
        };
    }
}
