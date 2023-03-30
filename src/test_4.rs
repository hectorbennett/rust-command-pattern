trait Command {
    fn execute(&self);
}

struct History {
    revision: usize,
    stack: Vec<Box<dyn Command>>,
}

impl History {
    fn new() -> History {
        History {
            revision: 0,
            stack: Vec::new(),
        }
    }

    fn append(&mut self, cmd: Box<dyn Command>) {
        self.stack.truncate(self.revision);
        self.stack.push(cmd);
        self.revision += 1;
    }

    fn undo(&mut self) {
        if self.revision > 0 {
            self.revision -= 1;
        }
    }

    fn redo(&mut self) {
        if self.revision <= self.stack.len() {
            self.revision += 1;
        }
    }

    fn clear(&mut self) {
        self.stack.clear();
        self.revision = 0;
    }
}

impl Command for History {
    fn execute(&self) {
        for i in 0..self.revision {
            self.stack[i].execute();
        }
    }
}

struct DrawCommand {
    drawable: Box<dyn Drawable>,
    x: u32,
    y: u32,
}

impl DrawCommand {
    fn new(drawable: Box<dyn Drawable>, x: u32, y: u32) -> DrawCommand {
        DrawCommand {
            drawable: drawable,
            x: x,
            y: y,
        }
    }
}

impl Command for DrawCommand {
    fn execute(&self) {
        self.drawable.draw(self.x, self.y);
    }
}

trait Drawable {
    fn draw(&self, x: u32, y: u32);
}

#[derive(Clone)]
struct DrawCanvas {}

impl DrawCanvas {
    fn new() -> DrawCanvas {
        DrawCanvas {}
    }
}

impl Drawable for DrawCanvas {
    fn draw(&self, x: u32, y: u32) {
        println!("draw(x:{}, y:{})", x, y);
    }
}

pub fn test_4() {
    let mut history = History::new();
    let canvas = Box::new(DrawCanvas::new());

    // TODO
    let cmd1 = Box::new(DrawCommand::new(canvas.clone(), 1, 1));
    history.append(cmd1);

    let cmd2 = Box::new(DrawCommand::new(canvas.clone(), 2, 2));
    history.append(cmd2);

    println!("----------");
    history.execute();
    println!();

    println!("---undo---");
    history.undo();
    history.execute();
    println!();

    println!("--redo--");
    history.redo();
    history.execute();
    println!();

    println!("--undo--");
    history.undo();
    history.execute();
    println!();

    println!("--execute--");
    let cmd3 = Box::new(DrawCommand::new(canvas.clone(), 3, 3));
    history.append(cmd3);
    history.execute();
    println!();

    println!("---clear---");
    history.clear();
    history.execute();

}
