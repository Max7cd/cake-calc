#![allow(non_snake_case)]
use std::{f64::consts::PI, collections::HashMap};

// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::{prelude::*, core::AttributeValue};

mod translation;
use translation::{translate, Language};

fn main() {
    // launch the web app
    dioxus_web::launch(App);
}

struct KeepHeight(bool);

fn App(cx: Scope) -> Element {
    let cake1 = use_ref(cx, CakeState::new);
    let cake2 = use_ref(cx, CakeState::new);
    let factor = use_state(cx, || 0.0f64);
    use_shared_state_provider(cx, || Language::English);
    let language = use_shared_state::<Language>(cx).unwrap();
    use_shared_state_provider(cx, || KeepHeight(false));
    let keep_height = use_shared_state::<KeepHeight>(cx).unwrap();

    cx.render(rsx!(
        link {
            rel: "stylesheet",
            href: "style.css"
        }
        h1 { translate("Cake Size Converter", &language.read()) },
        div {
            class: "tile",
            display: "flex",
            flex_direction: "column",
            max_width: "300px",
            h2 {
                margin: "5px",
                translate("Settings", &language.read())
            },
            p { 
                margin: "5px",
                translate("Language", &language.read())
            },
            select {
                margin: "5px",
                onchange: move |event| { 
                    *language.write() = (&event.value[..]).into();
                },
                option { Language::English.to_string() },
                option { Language::German.to_string() },
            },
            label {
                margin: "5px",
                input {
                    r#type: "checkbox",
                    value: "{keep_height.read().0}",
                    onchange: move |event| {
                        (*keep_height.write()).0 = match &event.value[..] {
                            "true" => true,
                            _ => false
                        };
                        cake1.write().updateVolume(keep_height);
                        cake2.write().updateVolume(keep_height);
                        factor.set(cake2.read().volume / cake1.read().volume);
                        
                    }
                },
                "{translate(\"Same Height\", &language.read())}"
            },
        }
        div {
            display: "flex",
            flex_direction: "row",
            flex_wrap: "wrap",
            CakeEntry { 
                cakeState: cake1,
                onupdate: |_| { factor.set(cake2.read().volume / cake1.read().volume)}
            },
            CakeEntry { 
                cakeState: cake2,
                onupdate: |_| { factor.set(cake2.read().volume / cake1.read().volume)}
            },
            div {
                class: "tile",
                translate("Factor:", &language.read()),
                input {
                    margin_top: "5px",
                    margin_bottom: "5px",
                    r#type: "text",
                    value: "{factor.get():.2}",
                    disabled: true
                }
            }
        }
    ))
}


#[derive(PartialEq, Default)]
enum CakeType {
    #[default]
    Round,
    Ring,
    Rectangle,
    Custom
}

impl From<&CakeType> for String {
    fn from(value: &CakeType) -> Self {
        match value {
            CakeType::Round => "Round".into(),
            CakeType::Ring => "Ring".into(),
            CakeType::Rectangle => "Rectangle".into(),
            CakeType::Custom => "Custom".into(),
        }
    }
}

impl From<&str> for CakeType {
    fn from(value: &str) -> Self {
        match &value.to_lowercase()[..] {
            "round" => CakeType::Round,
            "ring" => CakeType::Ring,
            "rectangle" => CakeType::Rectangle,
            "custom" => CakeType::Custom,
            _ => panic!()
        }
    }
}

impl ToString for CakeType {
    fn to_string(&self) -> String {
        String::from(self)
    }
}

#[derive(PartialEq)]
struct CakeState {
    caketype: CakeType,
    volume: f64,
    values: HashMap<String, f64>,
}

impl CakeState {
    fn new() -> Self {
        let mut s = Self {
            caketype: Default::default(),
            volume: Default::default(),
            values: HashMap::new()
        };
        s.values.insert("Diameter".into(), 0.0);
        s.values.insert("Height".into(), 1.0);
        s.values.insert("Length".into(), 0.0);
        s.values.insert("Width".into(), 0.0);
        s.values.insert("Inner Diameter".into(), 0.0);
        s.values.insert("Outer Diameter".into(), 0.0);
        s.values.insert("Custom Volume".into(), 0.0);
        s
    }

    fn updateVolume<'a>(&mut self, state: UseSharedState<KeepHeight>) {
        let height;
        if state.read().0 {
            height = 1.0;
        } else {
            height = self.values["Height"];
        }

        match self.caketype {
            CakeType::Round => self.volume = height * (self.values["Diameter"] / 2.0) * (self.values["Diameter"] / 2.0) * PI,
            CakeType::Ring => self.volume = height * PI * ((self.values["Outer Diameter"] / 2.0) * (self.values["Outer Diameter"] / 2.0) - (self.values["Inner Diameter"] / 2.0) * (self.values["Inner Diameter"] / 2.0)),
            CakeType::Rectangle => self.volume = self.values["Width"] * self.values["Length"] * height,
            CakeType::Custom => self.volume = self.values["Custom Volume"],
        }
    }

    fn generateHTML<'a>(&self, cx: &'a Scoped<'a, CakeEntryProps>) -> Element<'a> {
        match self.caketype {
            CakeType::Round => cx.render(rsx! {
                self.generateSection(cx, "Diameter"),
                self.generateSection(cx, "Height")
            }),
            CakeType::Ring => cx.render(rsx! {
                self.generateSection(cx, "Inner Diameter"),
                self.generateSection(cx, "Outer Diameter"),
                self.generateSection(cx, "Height")
            }),
            CakeType::Rectangle => cx.render(rsx! {
                self.generateSection(cx, "Width"),
                self.generateSection(cx, "Length"),
                self.generateSection(cx, "Height")
            }),
            CakeType::Custom => cx.render(rsx! {
                self.generateSection(cx, "Custom Volume"),
            }),
        }
    }

    fn generateSection<'a>(&self, cx: &'a Scoped<'a, CakeEntryProps>, name: &'a str) -> Element<'a> {
        let language = use_shared_state::<Language>(cx).unwrap();
        let input_value = use_state(cx, || format!("{:.2}", self.values[name]));
        let keep_height = use_shared_state::<KeepHeight>(cx).unwrap();
        if keep_height.read().0 && name == "Height" {
            println!("test");
            return None
        }
        cx.render(rsx! {
            translate(name, &language.read()),
            input {
                r#type: "text",
                value: "{input_value}",
                margin_top: "5px",
                margin_bottom: "5px",
                oninput: move |event| { 
                    input_value.set(event.value.clone());
                    *cx.props.cakeState.write().values.get_mut(name).unwrap() = event.value.parse().unwrap_or(f64::NAN);
                    cx.props.cakeState.write().updateVolume(keep_height);
                    cx.props.onupdate.call(())
                }
            }
        })
    }
}

#[inline_props]
#[allow(unused_variables)] // onupdate is implicitly used by generateHTML
fn CakeEntry<'a>(cx: Scope<'a>, cakeState: &'a UseRef<CakeState>, onupdate: EventHandler<'a, ()>) -> Element<'a> {
    let language = use_shared_state::<Language>(cx).unwrap();
    let o1_lbl = translate(&CakeType::Round.to_string(), &language.read());
    let o1_val = CakeType::Round.to_string();
    let o2_lbl = translate(&CakeType::Rectangle.to_string(), &language.read());
    let o2_val = CakeType::Rectangle.to_string();
    let o3_lbl = translate(&CakeType::Ring.to_string(), &language.read());
    let o3_val = CakeType::Ring.to_string();
    let o4_lbl = translate(&CakeType::Custom.to_string(), &language.read());
    let o4_val = CakeType::Custom.to_string();
    cx.render(rsx! {
        div {
            class: "tile",
            translate("Shape", &language.read()),
            select {
                margin_top: "5px",
                margin_bottom: "5px",
                onchange: |event| { cakeState.with_mut(|state| { 
                    state.caketype = CakeType::from(&event.value[..]);
                });},
                option { 
                    label: "{o1_lbl}",
                    value: "{o1_val}"
                },
                option { 
                    label: "{o2_lbl}",
                    value: "{o2_val}"
                },
                option { 
                    label: "{o3_lbl}",
                    value: "{o3_val}"
                },
                option { 
                    label: "{o4_lbl}",
                    value: "{o4_val}"
                }
            },
            cakeState.read().generateHTML(cx),
            translate("Volume", &language.read()),
            input {
                r#type: "text",
                disabled: true,
                margin_top: "5px",
                margin_bottom: "5px",
                value: "{cakeState.read().volume:.2}"
            },
            img {
                padding: "15px",
                src: "cake-{cakeState.read().caketype.to_string().to_lowercase()}.png"
            }
        }
    })
}