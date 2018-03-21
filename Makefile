# **************************************************************************** #
#                                                                              #
#                                                         :::      ::::::::    #
#    Makefile                                           :+:      :+:    :+:    #
#                                                     +:+ +:+         +:+      #
#    By: hsabouri <hsabouri@student.42.fr>          +#+  +:+       +#+         #
#                                                 +#+#+#+#+#+   +#+            #
#    Created: 2042/01/01 10:32:42 by hsabouri          #+#    #+#              #
#    Updated: 2018/03/21 17:17:38 by wescande         ###   ########.fr        #
#                                                                              #
# **************************************************************************** #

NAME = npuzzle

red="\033[31m"
blue="\033[34m"
reset="\033[0m"

all: $(NAME)

$(NAME):
	@echo $(blue)"Compiling..."$(reset)
	@cargo build --release
	@ln -s target/release/npuzzle

clean: 
	@rm -rf npuzzle
	@echo $(red)"Deleted symlink"$(reset)
	@rm -rf target
	@echo $(red)"Deleting target directory"$(reset)

re: clean all
.PHONY: all clean re
