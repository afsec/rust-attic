trait Plug {
    type PlugType;

    fn connect(&self, plug: impl Plug<PlugType = Self::PlugType>) -> bool {
        true
    }

    fn adapt<P>(&self, plug: P) -> Adapter<P> {
        Adapter::new(plug)
    }
}

struct Adapter<P> {
    plug: P,
}

impl<P> Adapter<P> {
    fn new(plug: P) -> Adapter<P> {
        Adapter { plug }
    }
}

impl<P> Plug for Adapter<P> {
    type PlugType = P;
    fn connect(&self, plug: impl Plug<PlugType = Self::PlugType>) -> bool {
        true
    }
}

struct USB;
struct USBc;

impl Plug for USB {
    type PlugType = Self;
}

impl Plug for USBc {
    type PlugType = Self;
}

fn main() {
    let usb = USB;
    let usb_c = USBc;

    USB.connect(USB);
    USBc.connect(USBc);

    // USBc.connect(USB);
    // USB.connect(usb_c);

    usb.adapt(USBc).connect(usb_c);

    println!("Hello, world!");
}
