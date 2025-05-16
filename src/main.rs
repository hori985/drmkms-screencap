mod card;
mod utils;
// use std::{os::unix::io::{AsFd, BorrowedFd}};

use drm::control::{connector, crtc, plane};

use crate::card::*;
use crate::utils::*;

fn get_active_connector(card : &Card) -> Option<connector::Info>{
    let resources = card.resource_handles().unwrap();

    for &handle in resources.connectors() {
        let info = card.get_connector(handle, false).unwrap();
        if info.current_encoder() != None {
            return Some(info);
        }
    }

    None
}

fn get_plane_of_crtc(card: &Card, crtc : &crtc::Handle, planes: Vec<plane::Handle>) -> Option<plane::Info> {
    for handle in planes {
        let info = card.get_plane(handle).unwrap();
        if let Some(crtc_handle) = info.crtc()  {
            if crtc_handle == *crtc {
                return Some(info);
            }
        }
    }

    // for handle in planes {
    //     let info = card.get_plane(handle).unwrap();
    //     println!("Plane: {:?}", handle);
    //     println!("\tCRTC: {:?}", info.crtc());
    //     println!("\tFramebuffer: {:?}", info.framebuffer());
    //     println!("\tFormats: {:?}", info.formats());
    // }


    None
}

pub fn main() {
    let card = Card::open_global();
    let resources = card.resource_handles().unwrap();

    // get  connector -> encoder -> crtc -> plane -> fb ?

    // Enable all possible client capabilities
    for &cap in capabilities::CLIENT_CAP_ENUMS {
        if let Err(e) = card.set_client_capability(cap, true) {
            eprintln!("Unable to activate capability {:?}: {}", cap, e);
            return;
        }
    }

    let plane_res = card.plane_handles().unwrap();

    // Print out all card resource handles
    println!("Connectors:\t{:?}", resources.connectors());
    println!("Encoders:\t{:?}", resources.encoders());
    println!("CRTCs:\t\t{:?}", resources.crtcs());
    println!("Framebuffers:\t{:?}", resources.framebuffers());
    println!("Planes:\t\t{:?}", plane_res);

    println!("\n");

    // Print connector that is used
    let active_connector = get_active_connector(&card).unwrap();

    println!("Connector: \n{:#?}", active_connector);
    println!("\n");

    // Print active encoder of connector
    let active_encoder = card.get_encoder(active_connector.current_encoder().unwrap()).unwrap();

    println!("Encoder: \n{:#?}", active_encoder);
    println!("\n");

    let active_crtc = card.get_crtc(active_encoder.crtc().unwrap()).unwrap();

    println!("CRTC: \n{:#?}", active_crtc);
    println!("\n");

    let plane_54 = get_plane_of_crtc(&card, &active_crtc.handle(), plane_res).unwrap();

    println!("Plane: \n{:#?}", plane_54);
    println!("\n");

    let fb = card.get_framebuffer(plane_54.framebuffer().unwrap()).unwrap();

    println!("FB: \n{:#?}", fb);
    println!("\n");

    // println!("\n\nALL RESOURCES:\n");
    // list_resources(&card);

    // println!("\n\nALL PROPERTIES:\n");
    // list_all_properties(&card);
}