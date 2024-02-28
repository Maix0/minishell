/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   tokenize.h                                         :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: maiboyer <maiboyer@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/02/28 17:49:22 by maiboyer          #+#    #+#             */
/*   Updated: 2024/02/28 17:50:02 by maiboyer         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#ifndef TOKENIZE_H
#define TOKENIZE_H

#include "lexer/lexer_types.h"
#include "me/types.h"
#include "me/vec/vec_token.h"

t_error tokenize(t_const_str s, t_vec_token *out);

#endif /* TOKENIZE_H */
