# AGENTS.md

Este repositorio pertenece a Jeresoft Academy y se rige por RFC-0001 y
RFC-0002. Es un curso de criptografía en Rust: enseña mecanismos, límites y
criterio de ingeniería, no recetas para crear criptografía de producción.

## Reglas

- Explicar concepto, problema, alternativas, decisión e invariantes antes del código.
- Rust estable; sin `unsafe`, nightly ni dependencias no triviales sin autorización.
- Las primitivas implementadas son material educativo; producción usa bibliotecas auditadas y protocolos establecidos.
- TDD, `cargo fmt`, Clippy, pruebas y doctests en verde.
- Español es-MX correcto y contenido en estado `draft`.
- Cada issue y PR comparte asignación a `jeresoftx`, milestone, labels y Project.
- No afirmar propiedades de seguridad que no estén demostradas por el modelo, pruebas y límites del capítulo.
