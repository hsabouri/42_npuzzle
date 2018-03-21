# **************************************************************************** #
#                                                                              #
#                                                         :::      ::::::::    #
#    Makefile                                           :+:      :+:    :+:    #
#                                                     +:+ +:+         +:+      #
#    By: hsabouri <hsabouri@student.42.fr>          +#+  +:+       +#+         #
#                                                 +#+#+#+#+#+   +#+            #
#    Created: 2042/01/01 10:32:42 by hsabouri          #+#    #+#              #
#    Updated: 2042/01/01 16:55:31 by hsabouri         ###   ########.fr        #
#                                                                              #
# **************************************************************************** #

NAME = npuzzle

ccred=$(echo -e "\x1b[31m")
ccblue=$(echo -e "\x1b[34m")
ccreset=$(echo -e "\x1b[0m")

.PHONE: all
all: $(NAME)

$(NAME):
	$(ccblue)
	@echo "Compiling..."
	$(ccreset)
	@cargo build --release
	@ln -s target/release/npuzzle

.PHONE: clean
clean: 
	@rm -rf npuzzle
	$(ccred)
	@echo "Deleted symlink"
	$(ccreset)
	@rm -rf target
	$(ccred)
	@echo "Deleting target directory"
	$(ccreset)

.PHONE: re
re: clean all
