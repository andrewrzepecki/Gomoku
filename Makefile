NAME = my_crate

all: $(NAME)

$(NAME):
	cargo build

clean:
	cargo clean

fclean: clean
	cargo clean --release

re: fclean all
