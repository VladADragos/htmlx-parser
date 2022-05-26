use nom::{
    IResult,
    error::{Error,ErrorKind,ParseError},
    bytes::complete::{tag, take_while, take_while1, take_while_m_n,take,is_a,take_until},
    combinator::map_res,
    sequence::{delimited, preceded, terminated, tuple,pair},
};



pub enum Either<A,B> {
    Left(A),
    Right(B),
}

#[derive(Debug, PartialEq)]
pub enum ElementKind {
   Div,H1,Text
}

#[derive(Debug, PartialEq)]
pub struct Element {
    pub kind: ElementKind,
    pub children: Vec<Element>,
}



impl Element {
    pub fn new(kind: ElementKind) -> Element {
        Element {
            kind,
            children: Vec::new(),
        }
    }
    pub fn new_with_children(kind: ElementKind,children: Vec<Element>) -> Element {
        Element {
            kind,
            children,
        }
    }
}

// fn parse_element_kind(input: &str) -> IResult<&str, ElementKind> {
//     let (input, _) = tag("<")(input)?;
//     let (input, kind) = take_while1(|c| c != '>')(input)?;
//     let (input, _) = tag(">")(input)?;
//     let kind = match kind {
//         "div" => ElementKind::Div,
//         "h1" => ElementKind::H1,
//         "text" => ElementKind::Text,
//         _ => return Err(nom::Err::Error((input, nom::error::ErrorKind::Tag))),
//     };
//     Ok((input, kind))
// }

// fn parse_element(input: &str) -> IResult<&str, Element> {
//     let (input, kind) = parse_element_kind(input)?;
//     let (input, children) = parse_element_children(input)?;
//     Ok((input, Element::new_with_children(kind, children)))
// }


fn parser(s: &str) -> IResult<&str,Option<Element>> {
    println!("parsing {}",s);
    if s.len() == 0 {
        return Ok((s,None));
    }

    let (restOfString,takenToken) = take(1usize)(s)?;
    
    if takenToken == "<" {
        println!("here0");
        let (rest,element_name) = take_while(|c| c != '>')(restOfString)?;
        let (rest,_) = tag(">")(rest)?;
        println!("here1");
    
    
        println!("here2");
        let (rest3,between2) = take_while(|c| c != '<')(rest)?;
        
        println!("it fails here");  


        println!("rest3 is  {:?}",rest3);
        let (a,b) = take_until("</")("<div><h1>text</h1></div>")?;
        let (c,_) = tag("</")(a)?;
        let (_,b) = take_until(">")(c)?;
    //    let mut p = delimited(tag("<"),tag("hello"),tag(">"));
    //    let (a,b) = p("<hello>rest of the string")?;
       println!("results:::A {:?}",a);
       println!("results:::B {:?}",b);

        // remove </
        let (rest4,_) = tag("</")(rest3)?; 
    
        // remove >
        let (rest5,between3) = take_while(|c| c != '>')(rest4)?;
    
        // check if end tag matches start tag
        if between3 == element_name {
            println!("they are the same");
        }
    
        let kind = match element_name {
            "div" => ElementKind::Div,
            "h1" => ElementKind::H1,
            "text" => ElementKind::Text,
            _ => return Err(nom::Err::Error(nom::error::Error::new(rest, nom::error::ErrorKind::Tag))),
        };
    
        if between2.len() == 0{
            Ok((between2,Some(Element::new( kind))))    
        } else {
    
            let (rest6,children) = parser(between2)?;
            if let Some(children) = children {
                Ok((rest6,Some(Element::new_with_children(kind,vec!(children)))))
            } else {
                panic!("cant this happen?");
            }
            // Ok((rest6,Some(Element::new_with_children( kind, children))))
        }
    }else{
        let x = takenToken.chars().nth(0).unwrap();
        if x.is_alphabetic(){
            Ok((restOfString,Some(Element::new(ElementKind::Text))))
        }else{
            Err(nom::Err::Error(nom::error::Error::new(takenToken, nom::error::ErrorKind::Tag)))
        }
    }






    


    
  }

  fn parse2(s: &str) -> IResult<&str,Option<Element>> {
    println!("parsing {:?}",s);
    if s.len() == 0 {
        return Ok((s,None));
    }

    // let (rest,takenToken) = take(1usize)(s)?;

    let firstChar = s.chars().nth(0).unwrap();


    if  firstChar == '<' {
        let (rest,_) = tag("<")(s)?;
        let (restOfString,element) = take_until(">")(rest)?;
        
        let (restOfString,_) = tag(">")(restOfString)?;
        println!("rest of string is {:?}",restOfString);
        println!("element is {:?}",element);
        
        // let x = take_until("</");
        // let mut p2 = pair(tag("<h1>"),x);

        let (endTag,between) = take_until("<")(restOfString)?;
        let (x,y) = parse2(between)?;
        println!("x: {:?},y: {:?} ",x,y);
        // let (_,_) = qp2(s)?;
        
        let kind = match element {
            "div" => ElementKind::Div,
            "h1" => ElementKind::H1,
            _ => ElementKind::Text,
        };
        
        Ok((rest,Some( Element::new_with_children(kind, vec!(y.unwrap())))))

    }else{

        // check if text else error
        if firstChar.is_alphabetic(){
            Ok((s,Some(Element::new(ElementKind::Text))))
        }else{
           return Ok((s,Some(Element::new(ElementKind::Text))));
            
        }
    }

    // Ok(("",None))
  }

fn main() {
    let s : &str = "<div><h1>something</h1></div>";
    let (_,x) = parse2(s).unwrap();  

    // let element =Element::new_with_children(ElementKind::Div,vec!(Element::new(ElementKind::H1)));
    println!("PARSE OUTPUT >>>>  {:?}", x);
}
