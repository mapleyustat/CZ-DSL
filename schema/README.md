# Schema

One conceptual model, two encodings: `cz-dsl.xsd` (XML) and
`cz-dsl.schema.json` (JSON). Both encode the same conceptual model of the
language specification (docs/01) so the two encodings cannot drift apart.
Every unit element carries an SI dimension attribute, and three consistency
rules are enforced at schema level (units matched by dimension, conversion
path required for a differing coordinate reference, classification code
matching its declared level). Instances: `docs/examples/`.
