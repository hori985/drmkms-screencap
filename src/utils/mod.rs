use crate::card::*;

pub fn list_resources(card : &Card){
    // Enable all possible client capabilities
    for &cap in capabilities::CLIENT_CAP_ENUMS {
        if let Err(e) = card.set_client_capability(cap, true) {
            eprintln!("Unable to activate capability {:?}: {}", cap, e);
            return;
        }
    }

    let resources = card.resource_handles().unwrap();
    let plane_res = card.plane_handles().unwrap();

    // Print out all card resource handles
    println!("Connectors:\t{:?}", resources.connectors());
    println!("Encoders:\t{:?}", resources.encoders());
    println!("CRTCs:\t\t{:?}", resources.crtcs());
    println!("Framebuffers:\t{:?}", resources.framebuffers());
    println!("Planes:\t\t{:?}", plane_res);

    for &handle in resources.connectors() {
        let info = card.get_connector(handle, false).unwrap();
        println!("Connector: {:?}", handle);
        println!("\t{:?}-{}", info.interface(), info.interface_id());
        println!("\t{:?}", info.state());
        println!("\t{:?}", info.size());
        println!("\t{:?}", info.encoders());
        println!("\t{:?}", info.current_encoder());

        for mode in card.get_modes(handle).unwrap() {
            println!("{:?}", mode);
        }
    }
    println!("\n");

    for &handle in resources.encoders() {
        let info = card.get_encoder(handle).unwrap();
        println!("Encoder: {:?}", handle);
        println!("\t{:?}", info.kind());
        println!("\t{:?}", info.crtc());
    }
    println!("\n");

    for &handle in resources.crtcs() {
        let info = card.get_crtc(handle).unwrap();
        println!("CRTC: {:?}", handle);
        println!("\tPosition: {:?}", info.position());
        println!("\tMode: {:?}", info.mode());
        println!("\tFramebuffer: {:?}", info.framebuffer());
        println!("\tGamma Length: {:?}", info.gamma_length());
    }
    println!("\n");

    for &handle in resources.framebuffers() {
        let info = card.get_framebuffer(handle).unwrap();
        println!("Framebuffer: {:?}", handle);
        println!("\tSize: {:?}", info.size());
        println!("\tPitch: {:?}", info.pitch());
        println!("\tBPP: {:?}", info.bpp());
        println!("\tDepth: {:?}", info.depth());
    }

    println!("\n");

    for handle in plane_res {
        let info = card.get_plane(handle).unwrap();
        println!("Plane: {:?}", handle);
        println!("\tCRTC: {:?}", info.crtc());
        println!("\tFramebuffer: {:?}", info.framebuffer());
        println!("\tFormats: {:?}", info.formats());
    }
}

fn print_properties<T: drm::control::ResourceHandle>(card: &Card, handle: T) {
    let props = card.get_properties(handle).unwrap();

    for (&id, &val) in props.iter() {
        println!("Property: {:?}", id);
        let info = card.get_property(id).unwrap();
        println!("{:?}", info.name());
        println!("{:#?}", info.value_type());
        println!("Mutable: {}", info.mutable());
        println!("Atomic: {}", info.atomic());
        println!("Value: {:?}", info.value_type().convert_value(val));
        println!();
    }
}


pub fn list_all_properties(card : &Card) {
    // Enable all possible client capabilities
    for &cap in capabilities::CLIENT_CAP_ENUMS {
        if let Err(e) = card.set_client_capability(cap, true) {
            eprintln!("Unable to activate capability {:?}: {}", cap, e);
            return;
        }
    }

    let resources = card.resource_handles().unwrap();
    let plane_res = card.plane_handles().unwrap();

    for &handle in resources.connectors() {
        print_properties(&card, handle);
    }

    for &handle in resources.framebuffers() {
        print_properties(&card, handle);
    }

    for &handle in resources.crtcs() {
        print_properties(&card, handle);
    }

    for handle in plane_res {
        print_properties(&card, handle);
    }
}