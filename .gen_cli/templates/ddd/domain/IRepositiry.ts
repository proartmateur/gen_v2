$FILE_HEADER$

import { $ent$ } from './$ent$';
import { $ent$DTO } from './$ent$DTO';

/**
 * Repository interface for $ent$ entity
 */
export interface I$ent$Repository {
    /**
     * Find entity by ID
     */
    findById(id: string): Promise<$ent$ | null>;
    
    /**
     * Find all entities
     */
    findAll(): Promise<$ent$[]>;
    
    /**
     * Save entity
     */
    save(entity: $ent$): Promise<void>;
    
    /**
     * Create from DTO
     */
    create(dto: $ent$DTO): Promise<$ent$>;
    
    /**
     * Update entity
     */
    update(id: string, dto: $ent$DTO): Promise<$ent$>;
    
    /**
     * Delete entity
     */
    delete(id: string): Promise<void>;
}
