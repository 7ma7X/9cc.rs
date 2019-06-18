use crate::lexer::*;
use crate::parser::*;

/**
 * 抽象構文木からアセンブリを生成
 */
impl Node {
  pub fn gen(&self) {
    if let Tk::Num(n) = &self.ty {
      println!("    push {}", n);
      return;    
    } else {
      if let Some(ref lnode) = &self.lhs {
        lnode.gen();
      }
      if let Some(ref rnode) = &self.rhs {
        rnode.gen();
      }

      println!("    pop rdi");
      println!("    pop rax");

      match &self.ty {
        Tk::Plus => {
          println!("    add rax, rdi");
        }
        Tk::Minus => {
          println!("    sub rax, rdi");
        }
        Tk::Multi => {
          println!("    imul rdi");
        }
        Tk::Div => {
          println!("    cqo");
          println!("    idiv rdi");
        }
        rl @ Tk::Equ | rl @ Tk::Ne | rl @ Tk::Lt | rl @ Tk::Le => {
          println!("    cmp rax, rdi");
          match rl {
            Tk::Equ => println!("    sete al"),
            Tk::Ne  => println!("    setne al"),
            Tk::Lt  => println!("    setl al"),
            Tk::Le  => println!("    setle al"),
            _ => {}
          }
          println!("    movzb rax, al");          
        }
        _ => {}
      }

      println!("    push rax");
    }
  }
}