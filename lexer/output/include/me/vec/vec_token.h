/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   vec_token.h                                  :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: maiboyer <maiboyer@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2023/12/04 18:46:53 by maiboyer          #+#    #+#             */
/*   Updated: 2023/12/09 17:53:00 by maiboyer         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#ifndef VEC_TOKEN_H
#define VEC_TOKEN_H

#include "lexer/lexer_types.h"
#include "me/types.h"

typedef bool (*t_vec_token_sort_fn)(t_token *, t_token *);
typedef void (*t_free_token_item)(t_token);

typedef struct s_vec_token
{
	t_free_token_item free_func;
	t_usize					len;
	t_usize					capacity;
	t_token		   *buffer;
} t_vec_token;

t_vec_token vec_token_new(t_usize				  capacity,
									  t_free_token_item free_function);
t_error vec_token_push(t_vec_token *vec, t_token element);
t_error vec_token_push_front(t_vec_token *vec,
								   t_token	  element);
t_error vec_token_pop(t_vec_token *vec, t_token *value);
t_error vec_token_pop_front(t_vec_token *vec, t_token *value);
void	vec_token_free(t_vec_token vec);
t_error vec_token_reserve(t_vec_token *vec,
								t_usize			   wanted_capacity);
t_error vec_token_find(t_vec_token *vec,
							 bool (*fn)(const t_token *), t_usize *index);
t_error vec_token_find_starting(t_vec_token *vec,
									  bool (*fn)(const t_token *),
									  t_usize starting_index, t_usize *index);
t_error vec_token_all(t_vec_token *vec,
							bool (*fn)(const t_token *), bool *result);
t_error vec_token_any(t_vec_token *vec,
							bool (*fn)(const t_token *), bool *result);
void	vec_token_iter(t_vec_token *vec,
							 void (*fn)(t_usize index, t_token *value,
										void *state),
							 void *state);
void	vec_token_reverse(t_vec_token *vec);
void	vec_token_sort(t_vec_token		  *vec,
							 t_vec_token_sort_fn is_sorted);

#endif
