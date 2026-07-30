# Plan de implementación de Rust Crypto

**Estado:** draft

**Representación operativa:** [GitHub Project #23](https://github.com/users/jeresoftx/projects/23).

## Objetivo

Entregar un curso y crate educativo de criptografía en Rust que permita razonar
sobre confidencialidad, integridad, autenticidad, derivación de claves y
fronteras de protocolo. La meta no es producir una biblioteca criptográfica:
los modelos se implementan para comprender y todo uso real debe elegir
bibliotecas auditadas, protocolos actuales y revisión especializada.

## Alcance y límites

El curso cubre SHA-256, password hashing, AES, RSA, ECC, árboles de Merkle,
TLS, JWT y OAuth. SHA-256, AES-128 y un árbol de Merkle se implementan y
prueban como modelos educativos. RSA usa enteros pequeños para mostrar la
aritmética, no claves seguras ni padding; ECC se explica mediante un modelo
algebraico pequeño. Password hashing, TLS, JWT y OAuth se estudian como
contratos de seguridad y selección de implementaciones auditadas.

No se usa `unsafe`, nightly, FFI ni dependencias externas sin una autorización
posterior. Ningún capítulo debe recomendar los modelos del crate para proteger
datos reales.

## Arquitectura de entrega

Cada unidad técnica entrega, en este orden, una especificación, un modelo
probado y un capítulo completo. Las fases son: fundación; hashes y
representaciones; cifrado simétrico y contraseñas; clave pública y firmas;
integridad y protocolos; cierre editorial.

## Fases

1. **Fundación:** contrato criptográfico, amenazas, glosario inicial y crate sin dependencias. [x]
2. **Hashes:** SHA-256, resistencia a colisiones, codificación, password hashing y límites del modelo.
3. **Simétrica:** confidencialidad, AES-128 educativo, modos, nonces y gestión de contraseñas.
4. **Clave pública:** aritmética modular, RSA didáctico, ECC, firmas y límites de claves pequeñas.
5. **Integridad y protocolos:** Merkle, MAC, TLS, JWT, OAuth y composición segura de protocolos.
6. **Cierre:** referencias, ejercicios, soluciones, auditoría y estado editorial `draft`.

## Ruta crítica

Fundación → hashes → cifrado simétrico → clave pública → integridad y
protocolos → cierre. Ninguna fase introduce dependencias, `unsafe` ni afirma
seguridad de producción.

## Criterio de cierre

El curso está completo como `draft` cuando cada unidad incluya concepto,
problema, alternativas, invariantes, Mermaid, ejemplos, ejercicios,
soluciones, límites y referencias; los modelos estén probados; cada issue y PR
sea trazable en el Project; no haya pendientes ni milestones abiertos.

## Primer bloque ejecutable

Hashes: especificar SHA-256, codificación, sal y límites de password hashing
antes de implementar el modelo. No requiere dependencias, `unsafe` ni nightly.
