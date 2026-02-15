$FILE_HEADER$

/**
 * Value Object: $ent_prop$VO
 * Encapsulates the $prop$ property with validation
 */
export class $ent_prop$VO {
    private readonly _value: $prop_type$;

    constructor(value: $prop_type$) {
        this.validate(value);
        this._value = value;
    }

    get value(): $prop_type$ {
        return this._value;
    }

    private validate(value: $prop_type$): void {
        // TODO: Add validation rules for $prop$
        if (value === null || value === undefined) {
            throw new Error('$ent_prop$VO cannot be null or undefined');
        }
    }

    public equals(other: $ent_prop$VO): boolean {
        return this._value === other._value;
    }

    public toString(): string {
        return String(this._value);
    }
}
