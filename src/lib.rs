use zed_extension_api as zed;

struct GdslExtension {
    // ... state
}

impl zed::Extension for GdslExtension {
    fn new() -> Self {
        Self {}
    }
}

zed::register_extension!(GdslExtension);
