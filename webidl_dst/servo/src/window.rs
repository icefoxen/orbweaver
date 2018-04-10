use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern {
    type WindowBase64;
    #[wasm_bindgen(method)]
    pub fn btoa(this: &WindowBase64);
    #[wasm_bindgen(method)]
    pub fn atob(this: &WindowBase64);
}
#[wasm_bindgen]
extern {
    type Window;
    #[wasm_bindgen(method)]
    pub fn close(this: &Window);
    #[wasm_bindgen(method)]
    pub fn alert(this: &Window);
    #[wasm_bindgen(method)]
    pub fn requestAnimationFrame(this: &Window);
    #[wasm_bindgen(method)]
    pub fn cancelAnimationFrame(this: &Window);
    #[wasm_bindgen(method)]
    pub fn postMessage(this: &Window);
    #[wasm_bindgen(method)]
    pub fn captureEvents(this: &Window);
    #[wasm_bindgen(method)]
    pub fn releaseEvents(this: &Window);
    #[wasm_bindgen(method)]
    pub fn getComputedStyle(this: &Window);
    #[wasm_bindgen(method)]
    pub fn matchMedia(this: &Window);
    #[wasm_bindgen(method)]
    pub fn moveTo(this: &Window);
    #[wasm_bindgen(method)]
    pub fn moveBy(this: &Window);
    #[wasm_bindgen(method)]
    pub fn resizeTo(this: &Window);
    #[wasm_bindgen(method)]
    pub fn resizeBy(this: &Window);
    #[wasm_bindgen(method)]
    pub fn scroll(this: &Window);
    #[wasm_bindgen(method)]
    pub fn scrollTo(this: &Window);
    #[wasm_bindgen(method)]
    pub fn scrollBy(this: &Window);
    #[wasm_bindgen(method)]
    pub fn debug(this: &Window);
    #[wasm_bindgen(method)]
    pub fn gc(this: &Window);
    #[wasm_bindgen(method)]
    pub fn trap(this: &Window);
    #[wasm_bindgen(method)]
    pub fn webdriverCallback(this: &Window);
    #[wasm_bindgen(method)]
    pub fn webdriverTimeout(this: &Window);
}
#[wasm_bindgen]
extern {
    type WindowSessionStorage;
}
#[wasm_bindgen]
extern {
    type WindowTimers;
    #[wasm_bindgen(method)]
    pub fn setTimeout(this: &WindowTimers);
    #[wasm_bindgen(method)]
    pub fn clearTimeout(this: &WindowTimers);
    #[wasm_bindgen(method)]
    pub fn setInterval(this: &WindowTimers);
    #[wasm_bindgen(method)]
    pub fn clearInterval(this: &WindowTimers);
}
#[wasm_bindgen]
extern {
    type WindowProxy;
}
#[wasm_bindgen]
extern {
    type WindowLocalStorage;
}
