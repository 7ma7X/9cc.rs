use crate::tokenizer::*;
use crate::util::*;

#[derive(Debug)]
pub struct Node {
  ty: Tk,
  lhs: Option<Box<Node>>,
  rhs: Option<Box<Node>>
}

impl Node {
  fn new_node(ty: Tk, lhs: Option<Box<Node>>, rhs: Option<Box<Node>>) -> Node {
    Node { ty: ty, lhs: lhs, rhs: rhs }
  }

  fn new_node_num(val: i32) -> Node {
    Node { ty: Tk::Num(val), lhs: None, rhs: None }
  }

  /**
   * expr = mul ("+" mul | "-" mul)*
   */
  pub fn expr(tokens: &Vec<Token>, pos: &mut usize, original: &String) -> Node {
    let mut node: Node = Node::mul(tokens, pos, original);

    loop {
      if consume(Tk::Plus, tokens, pos) {
        node = Node::new_node(
          Tk::Plus, 
          Some(Box::new(node)), 
          Some(Box::new(Node::mul(tokens, pos, original)))
        )
      } else if consume(Tk::Minus, tokens, pos) {
        node = Node::new_node(
          Tk::Minus, 
          Some(Box::new(node)), 
          Some(Box::new(Node::mul(tokens, pos, original)))
        )
      } else {
        return node;
      }
    }
  }

  /**
   * mul  = term ("*" term | "/" term)*
   */
  fn mul(tokens: &Vec<Token>, pos: &mut usize, original: &String) -> Node {
    let mut node: Node = Node::term(tokens, pos, original).unwrap();

    loop {
      if consume(Tk::Multi, tokens, pos) {
        node = Node::new_node(
          Tk::Multi, 
          Some(Box::new(node)), 
          Some(Box::new(Node::mul(tokens, pos, original)))
        )
      } else if consume(Tk::Div, tokens, pos) {
        node = Node::new_node(
          Tk::Div, 
          Some(Box::new(node)), 
          Some(Box::new(Node::mul(tokens, pos, original)))
        ) 
      } else {
        return node;
      }
    }
  }


  /**
   * term = "(" expr ")" | num
   */
  fn term(tokens: &Vec<Token>, pos: &mut usize, original: &String) -> Result<Node, ()> {
    if consume(Tk::LParen, tokens, pos) {
      let node = Node::expr(tokens, pos, original);
      if !(consume(Tk::RParen, tokens, pos)) {
        error_at(&tokens[*pos].input, original, 
          "開きカッコに対応する閉じカッコがありません".to_string());
      }
      return Ok(node);
    }

    match tokens[*pos].ty {
      Tk::Num(n) => {
        *pos += 1;
        return Ok(Node::new_node_num(n));
      }
      _ => {
        error_at(&tokens[*pos].input, original,
          "数値でも開きカッコでもないトークンです".to_string());
        Err(())
      }
    }
  }

  /**
   * 抽象構文木からアセンブリを生成
   */
  pub fn gen(&self) {
    match &self.ty {
      Tk::Num(n) => {
        println!("    push {}", n);
        return;
      }
      _ => {
        match &self.lhs {
          Some(ref lnode) => { lnode.gen() }
          None => {}
        }
        match &self.rhs {
          Some(ref rnode) => { rnode.gen() }
          None => {}
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
          _ => {}
        }

        println!("    push rax");
      }
    }
  }
}

/**
 * 次のトークンが期待した型かどうかをチェックし, 
 * 期待した型の場合だけ入力を1トークン読み進めて真を返す関数
 */ 
fn consume(ty: Tk, tokens: &Vec<Token>, pos: &mut usize) -> bool {
  if tokens[*pos].ty != ty {
    return false;
  }
  *pos += 1;
  true
}