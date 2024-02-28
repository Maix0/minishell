/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   vec_token.c                                  :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: maiboyer <maiboyer@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2023/12/05 18:46:28 by maiboyer          #+#    #+#             */
/*   Updated: 2023/12/09 17:54:11 by maiboyer         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "me/mem/mem_alloc_array.h"
#include "me/mem/mem_copy.h"
#include "me/mem/mem_set_zero.h"
#include "me/types.h"
#include "me/vec/vec_token.h"
#include <stdlib.h>

t_vec_token vec_token_new(t_usize				  capacity,
									  t_free_token_item free_function)
{
	t_vec_token out;

	out = (t_vec_token){0};
	out.free_func = free_function;
	out.buffer = mem_alloc_array(capacity, sizeof(t_token));
	if (out.buffer)
		out.capacity = capacity;
	return (out);
}

/// Return true in case of an error
t_error vec_token_push(t_vec_token *vec, t_token element)
{
	t_token *temp_buffer;
	size_t		   new_capacity;

	if (vec == NULL)
		return (ERROR);
	if (vec->len + 1 > vec->capacity)
	{
		new_capacity = (vec->capacity * 3) / 2 + 1;
		while (vec->len + 1 > new_capacity)
			new_capacity = (new_capacity * 3) / 2 + 1;
		temp_buffer = mem_alloc_array(new_capacity, sizeof(t_token));
		if (temp_buffer == NULL)
			return (ERROR);
		mem_copy(temp_buffer, vec->buffer, vec->len * sizeof(t_token));
		free(vec->buffer);
		vec->buffer = temp_buffer;
		vec->capacity = new_capacity;
	}
	vec->buffer[vec->len] = element;
	vec->len += 1;
	return (NO_ERROR);
}

/// Return true in case of an error
t_error vec_token_reserve(t_vec_token *vec, t_usize wanted_capacity)
{
	t_token *temp_buffer;
	size_t		   new_capacity;

	if (vec == NULL)
		return (ERROR);
	if (wanted_capacity > vec->capacity)
	{
		new_capacity = (vec->capacity * 3) / 2 + 1;
		while (wanted_capacity > new_capacity)
			new_capacity = (new_capacity * 3) / 2 + 1;
		temp_buffer = mem_alloc_array(new_capacity, sizeof(t_token));
		if (temp_buffer == NULL)
			return (ERROR);
		mem_copy(temp_buffer, vec->buffer, vec->len * sizeof(t_token));
		free(vec->buffer);
		vec->buffer = temp_buffer;
		vec->capacity = new_capacity;
	}
	return (NO_ERROR);
}

/// Return true if the vector is empty
/// This function is safe to call with value being NULL
t_error vec_token_pop(t_vec_token *vec, t_token *value)
{
	t_token  temp_value;
	t_token *ptr;

	if (vec == NULL)
		return (ERROR);
	ptr = value;
	if (vec->len == 0)
		return (ERROR);
	if (value == NULL)
		ptr = &temp_value;
	vec->len--;
	*ptr = vec->buffer[vec->len];
	mem_set_zero(&vec->buffer[vec->len], sizeof(t_token));
	return (NO_ERROR);
}

/// This function is safe to call with `free_elem` being NULL
void vec_token_free(t_vec_token vec)
{
	if (vec.free_func)
	{
		while (vec.len)
		{
			vec.free_func(vec.buffer[vec.len - 1]);
			vec.len--;
		}
	}
	free(vec.buffer);
}
