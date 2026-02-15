$FILE_HEADER$

/**
 * Data Transfer Object for $ent$
 * Used for data transmission between layers
 */
export interface $ent$DTO {
(    $camel_prop$: $prop_type$;
)
}

/**
 * Create a new $ent$DTO
 */
export function create$ent$DTO(
(    $camel_prop$: $prop_type$,
)
): $ent$DTO {
    return {
(        $camel_prop$,
)
    };
}
