#![allow(unused)]

use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::os::unix::process::CommandExt;
use std::process::{exit, Command};
use std::vec;


mod lexer;
mod operations;

use operations::Operations;

#[derive(Debug)]
pub struct Stack<T> {
	pub items: Vec<T>,
}

fn crossreference_blocks(mut program: Vec<(Operations, i64)>) -> Vec<(Operations, i64)> {
    let mut stack_struct: Stack<i64> = Stack { items: Vec::new() };
    let mut stack = stack_struct.items;

    for ip in 0..program.len() {
        let op = &program[ip];
        if op.0 == Operations::If {
            stack.push(ip as i64);
        } else if op.0 == Operations::Else {
            let if_ip = stack.pop();
			program[if_ip.unwrap() as usize].1 = ip as i64 + 1;
			stack.push(ip as i64);
        } else if op.0 == Operations::End {
            let block_ip = stack.pop();
            if program[block_ip.unwrap() as usize].0 == Operations::If ||
               program[block_ip.unwrap() as usize].0 == Operations::Else {
                program[block_ip.unwrap() as usize] = (program[block_ip.unwrap() as usize].clone().0, ip as i64);
                program[ip] = (Operations::End, ip as i64 + 1);
            } else if program[block_ip.unwrap() as usize].0 == Operations::Do {
                program[ip] = (Operations::End, program[block_ip.unwrap() as usize].1);
                program[block_ip.unwrap() as usize] = (Operations::Do, ip as i64 + 1);
            } else {
                assert!(false, "'END' can only be used to close 'if-else' blocks for now");
            }
        } else if op.0 == Operations::While {
            stack.push(ip as i64);
        } else if op.0 == Operations::Do {
            let while_ip = stack.pop();
            program[ip] = (Operations::Do, while_ip.unwrap());

            stack.push(ip as i64);
        }
    }

    return program;
}

fn simulate_program(mut program: Vec<(Operations, i64)>) {
    let mut stack_struct: Stack<i64> = Stack { items: Vec::new() };
    let mut stack = stack_struct.items;
    
    let mut ip: usize = 0;
    while ip < program.len() {
        let op = &program[ip];
        if op.0 == Operations::Push {
            stack.push(op.1);
            ip += 1;
        } else if op.0 == Operations::Plus {
            let a = stack.pop();
            let b = stack.pop();
            let add = match (a, b) {
                (Some(x), Some(y)) => Some(x + y),
                _ => None,
            };

            stack.push(add.expect("Failed to add"));
            ip += 1;
        } else if op.0 == Operations::Minus {
            let a = stack.pop();
            let b = stack.pop();
            let minus = match (a, b) {
                (Some(x), Some(y)) => Some(y - x),
                _ => None,
            };

            stack.push(minus.expect("Failed to subtract"));
            ip += 1;
        } else if op.0 == Operations::Equal {
            let a = stack.pop();
            let b = stack.pop();
            let equals = match (a, b) {
                (Some(x), Some(y)) => Some(y == x),
                _ => None,
            };

            stack.push(equals.unwrap() as i64);
            ip += 1;
        } else if op.0 == Operations::GreaterThan {
            let a = stack.pop();
            let b = stack.pop();
            let equals = match (a, b) {
                (Some(x), Some(y)) => Some(y > x),
                _ => None,
            };

            stack.push(equals.unwrap() as i64);
            ip += 1;
        } else if op.0 == Operations::True {
            stack.push(1);
            ip += 1;
        } else if op.0 == Operations::False {
            stack.push(0);
            ip += 1;
        } else if op.0 == Operations::If {
            let a = stack.pop().unwrap_or(0);
            if a == 0 {
                ip = op.1 as usize;
            } else {
                ip += 1;
            }
        } else if op.0 == Operations::Else {
            ip = op.1 as usize;
        } else if op.0 == Operations::End {
            ip = op.1 as usize;
        } else if op.0 == Operations::While {
            ip += 1;
        } else if op.0 == Operations::Do {
            let a = stack.pop().unwrap();
            if a == 0 {
                ip = op.1 as usize;
            } else {
                ip += 1;
            }
        } else if op.0 == Operations::Dupl {
            let a = stack.pop();
            stack.push(a.unwrap());
            stack.push(a.unwrap());
            ip += 1;
        } else if op.0 == Operations::Dump {
            let a = stack.pop();
            println!("{}", a.unwrap());
            ip += 1;
        }
    }
}

fn compile_program(program: Vec<(Operations, i64)>, output_path: String) {

	let mut output = File::create(output_path).expect("Failed to create file!");
	output.write("bits 64\n".as_bytes());
	output.write("segment .text\n".as_bytes());

	// DUMP FUNCTION
	output.write("\ndump:\n".as_bytes());
    output.write("    mov     r9, -3689348814741910323\n".as_bytes());
    output.write("    sub     rsp, 40\n".as_bytes());
    output.write("    mov     BYTE [rsp+31], 10\n".as_bytes());
    output.write("    lea     rcx, [rsp+30]\n".as_bytes());
	output.write(".L2:\n".as_bytes());
    output.write("    mov     rax, rdi\n".as_bytes());
    output.write("    lea     r8, [rsp+32]\n".as_bytes());
    output.write("    mul     r9\n".as_bytes());
    output.write("    mov     rax, rdi\n".as_bytes());
    output.write("    sub     r8, rcx\n".as_bytes());
    output.write("    shr     rdx, 3\n".as_bytes());
    output.write("    lea     rsi, [rdx+rdx*4]\n".as_bytes());
    output.write("    add     rsi, rsi\n".as_bytes());
    output.write("    sub     rax, rsi\n".as_bytes());
    output.write("    add     eax, 48\n".as_bytes());
    output.write("    mov     BYTE [rcx], al\n".as_bytes());
    output.write("    mov     rax, rdi\n".as_bytes());
    output.write("    mov     rdi, rdx\n".as_bytes());
    output.write("    mov     rdx, rcx\n".as_bytes());
    output.write("    sub     rcx, 1\n".as_bytes());
    output.write("    cmp     rax, 9\n".as_bytes());
    output.write("    ja      .L2\n".as_bytes());
    output.write("    lea     rax, [rsp+32]\n".as_bytes());
    output.write("    mov     edi, 1\n".as_bytes());
    output.write("    sub     rdx, rax\n".as_bytes());
    output.write("    lea     rsi, [rsp+32+rdx]\n".as_bytes());
    output.write("    mov     rdx, r8\n".as_bytes());
	output.write("    mov     rax, 1\n".as_bytes());
    output.write("    syscall\n".as_bytes());
    output.write("    add     rsp, 40\n".as_bytes());
    output.write("    ret\n\n".as_bytes());

	output.write("global _start\n\n".as_bytes());
	output.write("_start:\n".as_bytes());
	for ip in 0..program.len() {
	    let op = &program[ip];
        
        output.write("addr_".as_bytes());
        output.write(ip.to_string().as_bytes());
        output.write(":\n".as_bytes());
        if op.0 == Operations::Push {
            output.write("    ;; ----- PUSH ----- ;;\n".as_bytes());
            output.write("    push ".as_bytes());
            output.write(op.1.to_string().as_bytes());
            output.write("\n".as_bytes());
        } else if op.0 == Operations::Plus {
            output.write("    ;; ----- PLUS ----- ;;\n".as_bytes());
            output.write("    pop rax\n".as_bytes());
            output.write("    pop rbx\n".as_bytes());
            output.write("    add rax, rbx\n".as_bytes());
            output.write("    push rax\n".as_bytes());
        } else if op.0 == Operations::Minus {
            output.write("    ;; ----- MINUS ----- ;;\n".as_bytes());
            output.write("    pop rax\n".as_bytes());
            output.write("    pop rbx\n".as_bytes());
            output.write("    sub rbx, rax\n".as_bytes());
            output.write("    push rbx\n".as_bytes());
        } else if op.0 == Operations::Equal {
            output.write("    ;; ----- EQUAL ----- ;;".as_bytes());
            output.write("    mov rcx, 0\n".as_bytes());
            output.write("    mov rdx, 1\n".as_bytes());
            output.write("    pop rax\n".as_bytes());
            output.write("    pop rbx\n".as_bytes());
            output.write("    cmp rax, rbx\n".as_bytes());
            output.write("    cmove rcx, rdx\n".as_bytes());
            output.write("    push rcx\n".as_bytes());
        } else if op.0 == Operations::GreaterThan {
            output.write("    ;; ----- GREATER THAN ----- ;;\n".as_bytes());
            output.write("    mov rcx, 0\n".as_bytes());
            output.write("    mov rdx, 1\n".as_bytes());
            output.write("    pop rbx\n".as_bytes());
            output.write("    pop rax\n".as_bytes());
            output.write("    cmp rax, rbx\n".as_bytes());
            output.write("    cmovg rcx, rdx\n".as_bytes());
            output.write("    push rcx\n".as_bytes());
        } else if op.0 == Operations::True {
            output.write("    mov rax, 1\n".as_bytes());
            output.write("    push rax\n".as_bytes());
        } else if op.0 == Operations::False {
            output.write("    mov rax, 0\n".as_bytes());
            output.write("    push rax\n".as_bytes());
        } else if op.0 == Operations::If {
            output.write("    ;; ----- IF STATEMENT ----- ;;\n".as_bytes());
            output.write("    pop rax\n".as_bytes());
            output.write("    test rax, rax\n".as_bytes());
            output.write("    jz addr_".as_bytes());
            output.write(op.1.to_string().as_bytes());
            output.write("\n".as_bytes());
        } else if op.0 == Operations::Else {
            output.write("    ;; ----- ELSE ----- ;;\n".as_bytes());
            output.write("    jmp addr_".as_bytes());
            output.write(op.1.to_string().as_bytes());
            output.write("\n".as_bytes());
        } else if op.0 == Operations::End {
            output.write("    ;; ----- END ----- ;;\n".as_bytes());
            if ip + 1 != op.1 as usize {
                output.write("    jmp addr_".as_bytes());
                output.write(op.1.to_string().as_bytes());
                output.write("\n".as_bytes());
            }
        } else if op.0 == Operations::While {
            output.write("    ;; ----- WHILE ----- ;;\n".as_bytes());
        } else if op.0 == Operations::Do {
            output.write("    ;; ----- DO ----- ;;\n".as_bytes());
            output.write("    pop rax\n".as_bytes());
            output.write("    test rax, rax\n".as_bytes());
            output.write("    jz addr_".as_bytes());
            output.write(op.1.to_string().as_bytes());
            output.write("\n".as_bytes());
        } else if op.0 == Operations::Dupl {
            output.write("    ;; ----- DUPL ----- ;;\n".as_bytes());
            output.write("    pop rax\n".as_bytes());
            output.write("    push rax\n".as_bytes());
            output.write("    push rax\n".as_bytes());
        } else if op.0 == Operations::Mem {
            output.write("    ;; ----- MEM ----- ;;\n".as_bytes());
            output.write("    push mem\n".as_bytes());
        } else if op.0 == Operations::Dump {
            output.write("    ;; ----- DUMP ----- ;;\n".as_bytes());
            output.write("    pop rdi\n".as_bytes());
            output.write("    call dump\n".as_bytes());
        } else {
            assert!(false, "Non-existant operation??");
        }
    }
	output.write("    mov rax, 60\n".as_bytes());
	output.write("    mov rdi, 0\n".as_bytes());
	output.write("    syscall\n".as_bytes());
    output.write("segment .bss\n".as_bytes());
    output.write("mem: resb ".as_bytes());
    output.write(operations::MEM_CAPACITY.to_string().as_bytes());
}

fn parse_token(row: usize, col: usize, word: String, file_path: String) -> (Operations, i64) {
    if word == "+" {
        return (Operations::Plus, 0);
    } else if word == "-" {
        return (Operations::Minus, 0);
    } else if word == "DUMP" {
        return (Operations::Dump, 0);
	} else if word == "=?" {
		return (Operations::Equal, 0);
    } else if word == ">" {
        return (Operations::GreaterThan, 0);
	} else if word == "TRUE" {
		return (Operations::True, 0);
	} else if word == "FALSE" {
		return (Operations::False, 0);
	} else if word == "IF" {
		return (Operations::If, 0);
	} else if word == "ELSE" {
		return (Operations::Else, 0);
	} else if word == "END" {
		return (Operations::End, 0);
    } else if word == "WHILE" {
		return (Operations::While, 0);
	} else if word == "DO" {
		return (Operations::Do, 0);
    } else if word == "DUPL" {
        return (Operations::Dump, 0);
    } else if word == "MEMORY" {
        return (Operations::Mem, 0);
    } else {
		match word.parse::<i64>() {
            Ok(parsed_value) => (Operations::Push, parsed_value),
            Err(e) => {
                eprintln!("{file_path}:{row}:{col}: {word}");
                (Operations::Push, 0);
				exit(1)
            }
        }
    }
}


fn load_program_from_file(program_file: &str) -> Vec<(Operations, i64)> {
    let mut program = Vec::new();

    if let Ok(tokens) = lexer::lex_file(program_file) {
        for (row_str, col, _, token) in tokens {
            let row: usize = row_str.parse().unwrap_or(0);
            program.push(parse_token(row, col, token, program_file.to_string()));
        }
    }

    crossreference_blocks(program)
}

		
fn main() {
	let args: Vec<String> = env::args().collect();

	
	if args.len() < 3 {
		println!("USAGE: tungsten-s <SUBCOMMAND> <FILE> <COM_ARGS>\n");
		println!("SUBCOMMANDS:");
		println!("     |- sim         - Simulate the program");
        println!("     |- build       - Compile the program");
        println!("     |- run         - Run & Compile the program");
		println!("ERROR: Did not specify a subcommand!");
		exit(1);
	}
	
	let subcommand = &args[1];
	let file_path = &args[2];

	if subcommand == "sim" {
        let program = load_program_from_file(file_path);
		simulate_program(program);
	} else if subcommand == "build" {
        let program = load_program_from_file(file_path);
		compile_program(program, "output.asm".to_string());
		Command::new("nasm").args(&["-felf64", "output.asm"]).output();
		Command::new("ld").args(&["-o", "output", "output.o"]).output();
	} else if subcommand == "run" {
        let program = load_program_from_file(file_path);
		compile_program(program, "output.asm".to_string());
		Command::new("nasm").args(&["-felf64", "output.asm"]).output();
		Command::new("ld").args(&["-o", "output", "output.o"]).output();
        Command::new("./output").exec();
	}
    
}
