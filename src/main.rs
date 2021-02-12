use blue_engine_core::{
    definitions::{Renderer, Vertex, WindowDescriptor},
    window,
};

fn test(renderer: &mut Renderer) {
    let VERTICIES: &[Vertex] = &[
        Vertex {
            position: [0.0, 1.0, 0.0],
            texture: [0.5, 0.0],
        },
        Vertex {
            position: [-1.0, -0.5, 0.0],
            texture: [0.0, 1.0],
        },
        Vertex {
            position: [1.0, -0.5, 0.0],
            texture: [1.0, 1.0],
        },
    ];

    let INDICIES = [0, 1, 2];

    let buffers = renderer.new_buffers(Vec::from(VERTICIES), Vec::from(INDICIES), 0..1);

    let cwd = std::env::current_dir().unwrap();
    let vertex_shader = std::fs::read(cwd.join("res").join("shader.vert.spv")).unwrap();
    let fragment_shader = std::fs::read(cwd.join("res").join("shader2.frag.spv")).unwrap();
    println!("WORKING 4");
    let shader = renderer
        .new_shaders("shader", vertex_shader, fragment_shader)
        .expect("Couldn't shader main");

    let pic = std::fs::read(cwd.join("res").join("profilePic50.png")).unwrap();

    println!("WORKING 1");

    let texture = renderer
        .new_texture("texture", Vec::from(pic), "clamp")
        .expect("couldn't load texture");

    println!("WORKING 2");

    renderer.new_pipeline(shader, buffers, Some(texture));
}

fn main() {
    /*
    let renderer = Renderer::new(window);

    // @TODO: make callback
    let window = win::new(800.0, 600.0, "Blue Engine", false, true, renderer);

    let cwd = std::env::current_dir().unwrap();
    let vertex_shader = std::fs::read(cwd.join("res").join("shader.vs.spv")).unwrap();
    let fragment_shader = std::fs::read(cwd.join("res").join("shader.fs.spv")).unwrap();

    let shader = renderer
        .new_shaders("shader", vertex_shader, fragment_shader)
        .expect("Couldn't shader main");

    let pipe1 = renderer.new_buffers(Vec::from(VERTICES), Vec::from(INDICES), 0..1);
    let pipe2 = renderer.new_buffers(
        Vec::from(TRIANGLES),
        Vec::from(TRAINGLEINDICES.clone()),
        0..1,
    );

    renderer.new_life(shader.clone(), pipe1);
    renderer.new_life(shader.clone(), pipe2);
    */

    window::new(WindowDescriptor {
        width: 800.0,
        height: 600.0,
        title: "title",
        decorations: true,
        resizable: true,
        before: Some(test),
        during: None,
        after: None,
    })
    .expect("win");

    /*
    let db = sled::open("data.db").expect("db open");
    db.insert("KEY1", "value");
    db.insert("KEY2", "value2");
    db.insert("KEY3", "value3");

    for i in db.range("KEY1".."KEY4") {
        println!(
            "{:?}",
            String::from_utf8(i.clone().unwrap().0.to_vec()).unwrap()
        );
        println!("{:?}", String::from_utf8(i.unwrap().1.to_vec()).unwrap());
    }
    println!("Size on disk: {}", db.size_on_disk().unwrap() / 1024);
    */
    /*{
        let lua = mlua::Lua::new();

        let file = std::fs::read("script.lua").unwrap();
        let chunk = lua.load(file.as_slice());
        chunk.exec();
        println!("{}", lua.used_memory() / 1000);
    }*/
}
