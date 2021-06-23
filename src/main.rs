use rltk::{ Rltk, GameState};

struct State {}
impl GameState for State {
    fn tick(&mut self , ctx : &mut Rltk){
        ctx.cls();
        ctx.print(1, 1, "Welcome to the rim");
    }
}


fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = rltk::RltkBuilder::simple80x50()
        .with_title("Rim Rogue")
        .build()?;
    let gs = State{ };
    rltk::main_loop(context, gs)
    
}
