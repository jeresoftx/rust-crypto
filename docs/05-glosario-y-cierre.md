# Glosario y cierre editorial

## Glosario

- **AEAD:** cifrado autenticado con datos asociados; aporta confidencialidad e
  integridad bajo el contrato de nonce del algoritmo.
- **AES:** cifrado simétrico de bloque de 128 bits; requiere un modo de
  operación para mensajes y no autentica por sí solo.
- **Digest:** salida de longitud fija de una función hash.
- **ECC:** criptografía de curva elíptica, basada en operaciones de grupo sobre
  curvas seleccionadas y auditadas.
- **Hash criptográfico:** función que resume datos para construcciones de
  integridad, compromiso y derivación; no cifra información.
- **KDF:** función de derivación de claves; para contraseñas se usa una KDF
  resistente a ataques de adivinación, con sal y costo configurable.
- **MAC:** código de autenticación de mensajes con secreto compartido.
- **Merkle:** árbol de hashes cuya raíz compromete hojas ordenadas bajo una
  regla de construcción.
- **Nonce:** valor que debe cumplir una regla de unicidad o imprevisibilidad
  según el protocolo; no es simplemente un número decorativo.
- **Padding:** esquema que estructura datos antes de RSA o una firma y evita
  vulnerabilidades del uso matemático directo.
- **RSA:** criptosistema de clave pública; su seguridad real requiere módulos
  grandes, padding y una implementación auditada.
- **Sal:** valor único asociado a una contraseña antes de usar una KDF.

## Referencias de estudio

- [NIST FIPS 180-4](https://csrc.nist.gov/pubs/fips/180-4/upd1/final): SHA-2.
- [NIST FIPS 197](https://csrc.nist.gov/pubs/fips/197/final): especificación de AES.
- [RFC 9106](https://www.rfc-editor.org/rfc/rfc9106): Argon2 y password hashing.
- [OWASP Password Storage Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/Password_Storage_Cheat_Sheet.html): decisiones operativas de contraseñas.
- [RFC 8446](https://www.rfc-editor.org/rfc/rfc8446): TLS 1.3.
- [RFC 8725](https://www.rfc-editor.org/rfc/rfc8725): mejores prácticas para JWT.
- [RFC 7636](https://www.rfc-editor.org/rfc/rfc7636): PKCE para OAuth.

## Auditoría de cierre

El curso cubre SHA-256, password hashing, AES, RSA, ECC, Merkle, TLS, JWT y
OAuth con una separación explícita entre mecanismo educativo y uso real. Los
modelos de SHA-256, AES-128, RSA de enteros pequeños y Merkle tienen pruebas
de contrato. El crate no usa `unsafe`, nightly ni dependencias externas.

La auditoría no reemplaza revisión humana: el contenido permanece en `draft`.
Antes de publicar, una persona debe revisar exactitud criptográfica, versiones
de referencias, ejemplos, enlaces y decisiones de integración para el entorno
que corresponda.

## Lista de verificación editorial

- [x] Cada unidad explica concepto, problema, alternativas e invariantes antes del código.
- [x] Los modelos tienen límites de producción explícitos.
- [x] Los capítulos incluyen Mermaid, ejemplos, ejercicios y soluciones.
- [x] Las primitivas de producción se remiten a bibliotecas y protocolos auditados.
- [x] El estado se conserva en `draft`, sin `reviewed` ni `published`.
