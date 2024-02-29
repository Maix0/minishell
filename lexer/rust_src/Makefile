
-include /home/nixos/school/minishell/lexer/rust_src/target/release/liblexer.d

/home/nixos/school/minishell/lexer/rust_src/target/release/liblexer.a: 
	cargo build --release --manifest-path /home/nixos/school/minishell/lexer/rust_src/Cargo.toml

all: /home/nixos/school/minishell/lexer/rust_src/target/release/liblexer.a

clean: 
	@rm -rf target

re: | clean all

%.rs: 
	cargo build --release --manifest-path /home/nixos/school/minishell/lexer/rust_src/Cargo.toml