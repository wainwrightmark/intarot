use std::{f64::consts, ops::Range};

use itertools::Itertools;

use rand::{rngs::ThreadRng, Rng};
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::{window, CanvasRenderingContext2d, HtmlCanvasElement};
use yew::{html, Component, Context, Html, NodeRef};

pub struct ParticlesCanvas {
    canvas: NodeRef,
    particles: Vec<Particle>,
    callback: Closure<dyn FnMut()>,

    request_id: Option<i32>,
}

impl Drop for ParticlesCanvas {
    fn drop(&mut self) {
        if let Some(handle) = self.request_id {
            if let Some(window) = window() {
                let _ = window.cancel_animation_frame(handle);
            }
        }
    }
}

impl Component for ParticlesCanvas {
    type Message = ();
    type Properties = ();
    fn create(ctx: &Context<Self>) -> Self {
        //info!("Created");
        let comp_ctx = ctx.link().clone();
        let callback =
            Closure::wrap(Box::new(move || comp_ctx.send_message(())) as Box<dyn FnMut()>);
        ctx.link().send_message(());

        let window = window().unwrap();
        let width = window.inner_width().unwrap().as_f64().unwrap();
        let height = window.inner_height().unwrap().as_f64().unwrap();

        Self {
            canvas: NodeRef::default(),
            particles: Particle::new_vec(100, width, height),
            callback,
            request_id: None,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _: Self::Message) -> bool {
        self.render();
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <canvas
                    id="canvas"
                    style="
            position: fixed;
        "
                    ref={self.canvas.clone()}>
                </canvas>
            </div>
        }
    }
}

impl ParticlesCanvas {
    fn render(&mut self) {
        let canvas: HtmlCanvasElement = self.canvas.cast().unwrap();

        let window = window().unwrap();
        let width = window.inner_width().unwrap().as_f64().unwrap().floor() as u32;
        let height = window.inner_height().unwrap().as_f64().unwrap().floor() as u32;

        canvas.set_width(width);
        canvas.set_height(height);

        let width = width as f64;
        let height = height as f64;

        let mut ctx: CanvasRenderingContext2d =
            canvas.get_context("2d").unwrap().unwrap().unchecked_into();

        // log::info!(
        //     "Render {} particles. width: {width:.3} height: {height:.3}",
        //     self.particles.len()
        // );

        ctx.clear_rect(0.0, 0.0, width, height);

        for particle in self.particles.iter_mut() {
            particle.update(width, height);
            particle.draw(&mut ctx);
        }

        let result = window.request_animation_frame(self.callback.as_ref().unchecked_ref());

        self.request_id = result.ok();
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Particle {
    x: Property,
    y: Property,
    size: Property,

    hue: Property,
    saturation: Property,
    lightness: Property,
    alpha: Property,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Property {
    pub value: f64,
    pub velocity: f64,
}

impl Property {
    pub fn new_in_range<R: Rng>(
        rng: &mut R,
        value_range: Range<f64>,
        velocity_range: Range<f64>,
    ) -> Self {
        Self {
            value: rng.gen_range(value_range),
            velocity: rng.gen_range(velocity_range),
        }
    }

    pub fn update_in_range(&mut self, range: Range<f64>) {
        if self.value < range.start && self.velocity.is_sign_negative() {
            self.velocity *= -1.;
        }

        if self.value > range.end && self.velocity.is_sign_positive() {
            self.velocity *= -1.;
        }

        self.value += self.velocity;
    }
}

const SIZE_RANGE: Range<f64> = 3.0..15.0;
const HUE_RANGE: Range<f64> = 60.0..120.0;
const SATURATION_RANGE: Range<f64> = 0.0..60.0;
const LIGHTNESS_RANGE: Range<f64> = 0.0..50.0;
const ALPHA_RANGE: Range<f64> = 0.08..0.12;

impl Particle {
    pub fn new_vec(count: usize, width: f64, height: f64) -> Vec<Self> {
        let mut rng = ThreadRng::default();

        (0..count)
            .map(|_| Particle::new(&mut rng, width, height))
            .collect_vec()
    }

    pub fn new<R: Rng>(rng: &mut R, width: f64, height: f64) -> Self {
        let x = Property::new_in_range(rng, 0.0..width, -0.5..0.5);
        let y = Property::new_in_range(rng, 0.0..height, -0.5..0.5);
        let size = Property::new_in_range(rng, SIZE_RANGE, 0.0..0.01);
        let hue = Property::new_in_range(rng, HUE_RANGE, 0.0..0.01);
        let saturation = Property::new_in_range(rng, SATURATION_RANGE, 0.0..0.01);
        let lightness = Property::new_in_range(rng, LIGHTNESS_RANGE, 0.0..0.01);
        let alpha = Property::new_in_range(rng, ALPHA_RANGE, 0.0..0.0001);

        Self {
            x,
            y,
            size,
            hue,
            saturation,
            lightness,
            alpha,
        }
    }

    pub fn update(&mut self, width: f64, height: f64) {
        self.x.update_in_range(0.0..width);
        self.y.update_in_range(0.0..height);
        self.size.update_in_range(SIZE_RANGE);
        self.hue.update_in_range(HUE_RANGE);
        self.saturation.update_in_range(SATURATION_RANGE);
        self.lightness.update_in_range(LIGHTNESS_RANGE);
        self.alpha.update_in_range(ALPHA_RANGE);
    }

    pub fn draw(&self, ctx: &mut CanvasRenderingContext2d) {
        let hue = self.hue.value;
        let saturation = self.saturation.value;
        let lightness = self.lightness.value;
        let alpha = self.alpha.value;
        //log::info!("{self:?}");
        ctx.begin_path();
        ctx.set_fill_style(&JsValue::from_str(
            format!("hsla({hue:.1}, {saturation:.1}%, {lightness:.1}%, {alpha:.2})").as_str(),
        ));
        ctx.ellipse(
            self.x.value,
            self.y.value,
            self.size.value,
            self.size.value,
            0.0,
            0.0,
            consts::TAU,
        )
        .unwrap();
        ctx.fill();
        ctx.close_path();

        //ctx.fill_rect(self.x, self.y, self.size * width_ratio, self.size * height_ratio);
    }
}
