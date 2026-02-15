$FILE_HEADER$

export interface $ent$Props{
(    $prop$: $prop_type$;
)
}

export function $ent$(props: $ent$Props){
    return (
        <div className="$kebab_name$">
            <h2>$ent$ Component</h2>
(            <p>$prop$: {props.$prop$}</p>
)
        </div>
    );
}
