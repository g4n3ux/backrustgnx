-- Add migration script here-- 1. Tabla de Roles
CREATE TABLE roles (
    id SERIAL PRIMARY KEY,
    nombre VARCHAR(50) UNIQUE NOT NULL
);

-- Insertar roles básicos iniciales
INSERT INTO roles (nombre) VALUES ('user'), ('admin');

-- 2. Tabla de Usuarios
CREATE TABLE usuario (
    id SERIAL PRIMARY KEY,
    nombre VARCHAR(100) NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    
    -- Auditoría
    creado_en TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    modificado_en TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    modificado_por INT -- ID del usuario que realizó el último cambio
);

-- 3. Tabla Intermedia (Relación Muchos a Muchos)
CREATE TABLE usuario_roles (
    usuario_id INT REFERENCES usuario(id) ON DELETE CASCADE,
    role_id INT REFERENCES roles(id) ON DELETE CASCADE,
    PRIMARY KEY (usuario_id, role_id)
);

-- 4. Trigger para actualizar 'modificado_en' automáticamente
CREATE OR REPLACE FUNCTION update_modified_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.modificado_en = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_usuario_modificado_en
    BEFORE UPDATE ON usuario
    FOR EACH ROW
    EXECUTE PROCEDURE update_modified_column();
