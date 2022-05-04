import Ajv from "ajv";

import { generateJsonSchema } from "./generateJsonSchema";
import { foxgloveMessageSchemas } from "./schemas";
import { exampleMessage } from "./testFixtures";

describe("generateJsonSchema", () => {
  it("generates expected JSON Schema", () => {
    expect(generateJsonSchema(exampleMessage)).toMatchInlineSnapshot(`
      Object {
        "$comment": "Generated from ExampleMessage by @foxglove/message-schemas",
        "description": "An example type",
        "properties": Object {
          "field_boolean": Object {
            "description": "boolean field",
            "type": "boolean",
          },
          "field_boolean_array": Object {
            "description": "boolean array field",
            "items": Object {
              "type": "boolean",
            },
            "type": "array",
          },
          "field_bytes": Object {
            "contentEncoding": "base64",
            "description": "bytes field",
            "type": "string",
          },
          "field_bytes_array": Object {
            "description": "bytes array field",
            "items": Object {
              "contentEncoding": "base64",
              "type": "string",
            },
            "type": "array",
          },
          "field_duration": Object {
            "description": "Duration field",
            "properties": Object {
              "nsec": Object {
                "type": "integer",
              },
              "sec": Object {
                "type": "integer",
              },
            },
            "title": "Duration",
            "type": "object",
          },
          "field_duration_array": Object {
            "description": "Duration array field",
            "items": Object {
              "properties": Object {
                "nsec": Object {
                  "type": "integer",
                },
                "sec": Object {
                  "type": "integer",
                },
              },
              "title": "Duration",
              "type": "object",
            },
            "type": "array",
          },
          "field_enum": Object {
            "description": "An enum field",
            "oneOf": Array [
              Object {
                "const": 1,
                "description": "Value A",
                "title": "A",
              },
              Object {
                "const": 2,
                "description": "Value B",
                "title": "B",
              },
            ],
            "title": "ExampleEnum: An example enum",
          },
          "field_enum_array": Object {
            "description": "An enum array field",
            "items": Object {
              "description": "An enum array field",
              "oneOf": Array [
                Object {
                  "const": 1,
                  "description": "Value A",
                  "title": "A",
                },
                Object {
                  "const": 2,
                  "description": "Value B",
                  "title": "B",
                },
              ],
              "title": "ExampleEnum: An example enum",
            },
            "type": "array",
          },
          "field_float": Object {
            "description": "float field",
            "type": "number",
          },
          "field_float_array": Object {
            "description": "float array field",
            "items": Object {
              "type": "number",
            },
            "type": "array",
          },
          "field_integer": Object {
            "description": "integer field",
            "type": "integer",
          },
          "field_integer_array": Object {
            "description": "integer array field",
            "items": Object {
              "type": "integer",
            },
            "type": "array",
          },
          "field_nested": Object {
            "$comment": "Generated from NestedMessage by @foxglove/message-schemas",
            "description": "A nested field",
            "properties": Object {
              "field_enum": Object {
                "description": "An enum field",
                "type": "integer",
              },
            },
            "title": "NestedMessage",
            "type": "object",
          },
          "field_nested_array": Object {
            "description": "A nested array field",
            "items": Object {
              "$comment": "Generated from NestedMessage by @foxglove/message-schemas",
              "description": "An example nested message",
              "properties": Object {
                "field_enum": Object {
                  "description": "An enum field",
                  "type": "integer",
                },
              },
              "title": "NestedMessage",
              "type": "object",
            },
            "type": "array",
          },
          "field_string": Object {
            "description": "string field",
            "type": "string",
          },
          "field_string_array": Object {
            "description": "string array field",
            "items": Object {
              "type": "string",
            },
            "type": "array",
          },
          "field_time": Object {
            "description": "Time field",
            "properties": Object {
              "nsec": Object {
                "type": "integer",
              },
              "sec": Object {
                "type": "integer",
              },
            },
            "title": "Time",
            "type": "object",
          },
          "field_time_array": Object {
            "description": "Time array field",
            "items": Object {
              "properties": Object {
                "nsec": Object {
                  "type": "integer",
                },
                "sec": Object {
                  "type": "integer",
                },
              },
              "title": "Time",
              "type": "object",
            },
            "type": "array",
          },
        },
        "title": "ExampleMessage",
        "type": "object",
      }
    `);
  });

  it.each(Object.values(foxgloveMessageSchemas))(
    "generates parseable JSON Schema for $name",
    (schema) => {
      const ajv = new Ajv();
      expect(() => ajv.compile(generateJsonSchema(schema))).not.toThrow();
    }
  );
});
