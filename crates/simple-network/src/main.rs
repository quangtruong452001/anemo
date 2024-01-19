use anemo::{types::PeerInfo, Network, Request, Response, Result, PeerId};
use bytes::Bytes;
use rand::Rng;
use std::{collections::HashMap, convert::Infallible};
use tower::{util::BoxCloneService, ServiceExt};
use tracing::trace;

#[tokio::main]
async fn main() {
    // Establish node
    let network_1 = build_network().unwrap();
    let network_2 = build_network().unwrap();
    let network_3 = build_network().unwrap();
    let network_4 = build_network().unwrap();
    let network_5 = build_network().unwrap();

  
    // Check local address of easch node
    // println!("Node 1 local address is: {}", network_1.local_addr() );
    // println!("Node 2 local address is: {}", network_2.local_addr() );
    // println!("Node 3 local address is: {}", network_3.local_addr() );
    // println!("Node 4 local address is: {}", network_4.local_addr() );
    // println!("Node 5 local address is: {}", network_5.local_addr() );

    // PeerId of Node
    let peer_id_1 = network_1.peer_id();
    let peer_id_2 = network_2.peer_id();
    let peer_id_3 = network_3.peer_id();
    let peer_id_4 = network_4.peer_id();
    let peer_id_5 = network_5.peer_id();

    // Peer Info
    let peer_info_1 = PeerInfo {
        peer_id: peer_id_1,
        affinity: anemo::types::PeerAffinity::High,
        address: vec![network_1.local_addr().into()],
    };

    let peer_info_2 = PeerInfo {
        peer_id: peer_id_2,
        affinity: anemo::types::PeerAffinity::High,
        address: vec![network_2.local_addr().into()],
    };

    let peer_info_3 = PeerInfo {
        peer_id: peer_id_3,
        affinity: anemo::types::PeerAffinity::High,
        address: vec![network_3.local_addr().into()],
    };

    let peer_info_4 = PeerInfo {
        peer_id: peer_id_4,
        affinity: anemo::types::PeerAffinity::High,
        address: vec![network_4.local_addr().into()],
    };

    let peer_info_5 = PeerInfo {
        peer_id: peer_id_5,
        affinity: anemo::types::PeerAffinity::High,
        address: vec![network_5.local_addr().into()],
    };

    let peer_info_collections = vec![
        peer_info_1,
        peer_info_2,
        peer_info_3,
        peer_info_4,
        peer_info_5,
    ];
    let node_peer_id_collections = vec![peer_id_1, peer_id_2, peer_id_3, peer_id_4, peer_id_5];
    let network_collections = [
        network_1.clone(),
        network_2.clone(),
        network_3.clone(),
        network_4.clone(),
        network_5.clone(),
    ];
    let peer_id_address_map = HashMap::from([
        (peer_id_1, network_1.local_addr()),
        (peer_id_2, network_3.local_addr()),
        (peer_id_3, network_3.local_addr()),
        (peer_id_4, network_4.local_addr()),
        (peer_id_5, network_5.local_addr()),
    ]);

    // Add to known_peers()
    for i in 0..5 {
        let network = &network_collections[i];
        let peer_id = network.peer_id();

        for (index, node_peer_id) in node_peer_id_collections.iter().enumerate() {
            // println!("{:#?}", peer_info_collections[i].clone());
            // Add other node info into node known_peers()
            if node_peer_id != &peer_id {
                // Add to node known_peers()
                network
                    .known_peers()
                    .insert(peer_info_collections[index].clone());
            }
        }
    }

    // println!(
    //     "Network 1 know peer: {:#?}",
    //     network_1.known_peers().get_all()
    // );

    //  Select a random node to send message
    let node = &network_collections[get_random_number()];

    // Connect and send message to other peer in node's known_peers()
    let peer_infos = node.known_peers().get_all();
    for peer_info in peer_infos {
        let node_peer_id: PeerId = peer_info.peer_id;
        let address = peer_id_address_map.get(&node_peer_id).unwrap();

        node.connect_with_peer_id(*address, node_peer_id).await.unwrap();
        let msg = format!("Greeting from node {} to node {}", node.peer_id(), node_peer_id);
        let response = node.rpc(node_peer_id, Request::new(msg.into())).await.unwrap();

        println!("{:#?}", response.into_body());
    }
}

fn get_random_number() -> usize {
    let mut rng = rand::thread_rng();
    let random_number: usize = rng.gen_range(0..=4);
    random_number
}

fn random_key() -> [u8; 32] {
    let mut rng = rand::thread_rng();
    let mut bytes = [0u8; 32];
    rand::RngCore::fill_bytes(&mut rng, &mut bytes[..]);
    bytes
}

fn build_network() -> Result<Network> {
    let network = Network::bind("localhost:0")
        .private_key(random_key())
        .server_name("test")
        .start(echo_service())?; // still dont understand what echo_service do, add to run :(
    trace!(
        address =% network.local_addr(),
        peer_id =% network.peer_id(),
        "starting network"
    );

    Ok(network)
}

fn echo_service() -> BoxCloneService<Request<Bytes>, Response<Bytes>, Infallible> {
    let handle = move |request: Request<Bytes>| async move {
        // trace!("received: {}", request.body().escape_ascii());
        let response = Response::new(request.into_body());
        Result::<Response<Bytes>, Infallible>::Ok(response)
    };

    tower::service_fn(handle).boxed_clone()
}
