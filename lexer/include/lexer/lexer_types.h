/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   lexer_types.h                                      :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: maiboyer <maiboyer@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/02/28 16:54:55 by maiboyer          #+#    #+#             */
/*   Updated: 2024/02/28 17:49:14 by maiboyer         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#ifndef LEXER_TYPES_H
#define LEXER_TYPES_H

#include "me/types.h"

typedef enum e_token_type
{
	LPARENS,	// '('
	RPARENS,	// ')'
	LBRACKET,	// '['
	RBRACKET,	// ']'
	LCURLY,		// '{'
	RCURLY,		// '}'
	DQ_STRING,	// "double quote string"
	SQ_STRING,	// 'single quote string'
	NQ_STRING,	//   no\ quote\ string
	BQ_STRING,	//   `backtick string`
	DOLAR_SIGN, // '$'
	LESS_SIGN,	// '<'
	GREAT_SIGN, // '>'
	PIPE_SIGN,	// '|'
	AND_SIGN,	// '&'
	SEMI_SIGN,	// ';'
	NEWLINE,	// '\n'
} t_token_type;

typedef struct s_token
{
	t_const_str	 input;
	t_usize		 start;
	t_usize		 end;
	t_token_type type;
} t_token;

#endif /* TYPES_H */
