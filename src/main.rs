use pest::{Parser, iterators::{Pairs, Pair}};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "csv.pest"]
pub struct CSVParser;


#[derive(Debug)]
pub struct FuncArg {
    pub name: String,
    pub arg_type: String
}

#[derive(Debug)]
pub enum AccessType {
    Public,
    Private
}

#[derive(Debug)]
pub enum AST {
    File {
        functions: Vec<AST>
    },
    Func {
        access: AccessType,
        name: String,
        args: Vec<FuncArg>
    }
}

fn parse_file(pairs: Pair<'_, Rule>, asts: &mut Vec<AST>) {
    let mut functions = Vec::new();
    let inner_rules = pairs.into_inner();

    for pair in inner_rules {
        match pair.as_rule() {
            Rule::func_def => {
                parse_func(pair, &mut functions);
            },
            _ => {}
        }
    }

    asts.push(AST::File { functions });

    println!("parse_file");
}

fn parse_func(pairs: Pair<'_, Rule>, asts: &mut Vec<AST>) {
    let mut inner_rules = pairs.into_inner();
    let mut access = AccessType::Private;
    let mut name = String::new();
    let mut args = Vec::new();

    for rule in inner_rules {

        match rule.as_rule() {
            Rule::func_name => name = rule.as_str().to_string(),
            Rule::keyword_public => access = AccessType::Public,
            Rule::func_arg => {
                let mut rules = rule.into_inner();
                
                args.push(FuncArg {
                    name: rules.next().unwrap().as_str().to_string(),
                    arg_type: rules.next().unwrap().as_str().to_string()
                })
            }
            _ => {}
        }
    }

    /*let access = inner_rules.find(|item| item.as_rule() == Rule::func_return_type);

    let name = inner_rules.find(|item| item.as_rule() == Rule::func_name);


    let return_type = inner_rules.find(|item| item.as_rule() == Rule::func_return_type);
    println!("return_type: {:#?}", return_type);

    let args = inner_rules.find(|item| item.as_rule() == Rule::func_args);
    println!("args: {:#?}", args);

    let body = inner_rules.find(|item| item.as_rule() == Rule::func_body);
    println!("body: {:#?}", body);*/
    println!("access: {:#?}", access);
    println!("name: {:#?}", name);
    println!("parse_func");

    asts.push(AST::Func { access, name, args });
}

fn main() {
    let mut asts: Vec<AST> = Vec::new();
    
    let successful_parse = CSVParser::parse(Rule::file, "


    pub fun data(): void {

    }
    fun my_func(a: i32): o23 :test").unwrap();
    println!("{:#?}", successful_parse);
    
    for pair in successful_parse {
        match pair.as_rule() {
            Rule::file => {
                parse_file(pair, &mut asts);
            },
            _ => {

            }
        }
    }

    println!("{:#?}", asts);
}

