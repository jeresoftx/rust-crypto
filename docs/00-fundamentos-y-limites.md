# Fundamentos y límites

## Concepto y problema

La criptografía transforma datos bajo supuestos explícitos sobre atacantes,
secretos, canales y participantes. Un algoritmo no aporta seguridad por tener
un nombre conocido: la propiedad buscada, el protocolo que lo integra y la
operación de sus claves determinan el resultado.

El problema inicial del curso es evitar dos errores simétricos: tratar la
criptografía como magia o tratar una implementación corta como sustituto de un
diseño revisado. Aquí se implementan piezas para observar sus invariantes; no
se construye un producto criptográfico.

## Modelo de amenazas mínimo

Antes de elegir una construcción, responde qué busca el atacante:

- leer información confidencial;
- modificar mensajes sin ser detectado;
- suplantar a un participante;
- reutilizar un mensaje válido;
- adivinar contraseñas débiles u obtener material de claves;
- explotar errores de integración, tiempos, nonces o validación.

Las propiedades no son intercambiables. Cifrar no autentica; hashear no cifra;
firmar no oculta el contenido. Un protocolo debe declarar cuál propiedad exige
y qué supuestos mantiene fuera del algoritmo.

## Contrato educativo

El crate usa Rust estable y no incluye `unsafe`, FFI, generación de
aleatoriedad, persistencia de claves ni red. Sus implementaciones permiten
seguir estados y comprobar vectores, pero no sustituyen auditoría, protección
contra canales laterales, gestión de secretos ni actualizaciones de seguridad.

En producción se eligen bibliotecas auditadas, versiones vigentes y protocolos
estandarizados. Para contraseñas se usa una KDF diseñada para ese fin; para
cifrado se usa una construcción autenticada; para TLS, JWT y OAuth se integra
una implementación madura en vez de reescribir el protocolo.

## Alternativas y decisión

Usar exclusivamente una biblioteca auditada es la elección correcta para un
sistema real. Implementar desde cero sigue siendo útil en un contexto
educativo: revela padding, bloques, rondas, aritmética modular y composición.
El curso adopta ambos principios sin mezclarlos: modelos locales para aprender,
bibliotecas auditadas para proteger.

## Lista de verificación

- [x] Toda propiedad de seguridad se formula contra una amenaza concreta.
- [x] El crate declara que no es apto para producción.
- [x] Las construcciones de producción se remiten a bibliotecas y protocolos auditados.
- [x] No hay `unsafe`, dependencias externas ni afirmaciones de seguridad no verificadas.
