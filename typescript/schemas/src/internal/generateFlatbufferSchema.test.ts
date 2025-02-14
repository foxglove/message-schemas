import { generateFlatbuffers } from "./generateFlatbufferSchema";
import { exampleEnum, exampleMessage } from "./testFixtures";

describe("generateFlatbuffers", () => {
  it("generates Message .fbs files", () => {
    expect(generateFlatbuffers(exampleMessage, [exampleEnum])).toMatchInlineSnapshot(`
      "// Generated by https://github.com/foxglove/foxglove-sdk

      include "ByteVector.fbs";
      include "Duration.fbs";
      include "NestedMessage.fbs";
      include "Time.fbs";

      namespace foxglove;

      /// An example enum
      enum ExampleEnum : ubyte {
        /// Value A
        A = 0,

        /// Value B
        B = 1,
      }
      /// An example type
      table ExampleMessage {
        /// duration field
        field_duration:Duration (id: 0);

        /// time field
        field_time:Time (id: 1);

        /// boolean field
        field_boolean:bool = true (id: 2);

        /// bytes field
        field_bytes:[uint8] (id: 3);

        /// float64 field
        field_float64:double = 1.0 (id: 4);

        /// uint32 field
        field_uint32:uint32 = 5 (id: 5);

        /// string field
        field_string:string = "string-type" (id: 6);

        /// duration array field
        field_duration_array:[Duration] (id: 7);

        /// time array field
        field_time_array:[Time] (id: 8);

        /// boolean array field
        field_boolean_array:[bool] (id: 9);

        /// bytes array field
        field_bytes_array:[ByteVector] (id: 10);

        /// float64 array field
        field_float64_array:[double] (id: 11);

        /// uint32 array field
        field_uint32_array:[uint32] (id: 12);

        /// string array field
        field_string_array:[string] (id: 13);

        /// duration fixed-length array field
        /// length 3
        field_duration_fixed_array:[Duration] (id: 14);

        /// time fixed-length array field
        /// length 3
        field_time_fixed_array:[Time] (id: 15);

        /// boolean fixed-length array field
        /// length 3
        field_boolean_fixed_array:[bool] (id: 16);

        /// bytes fixed-length array field
        /// length 3
        field_bytes_fixed_array:[ByteVector] (id: 17);

        /// float64 fixed-length array field
        /// length 3
        field_float64_fixed_array:[double] (id: 18);

        /// uint32 fixed-length array field
        /// length 3
        field_uint32_fixed_array:[uint32] (id: 19);

        /// string fixed-length array field
        /// length 3
        field_string_fixed_array:[string] (id: 20);

        /// An enum field
        field_enum:ExampleEnum (id: 21);

        /// An enum array field
        field_enum_array:[ExampleEnum] (id: 22);

        /// A nested field
        field_nested:foxglove.NestedMessage (id: 23);

        /// A nested array field
        /// With
        /// a
        /// very
        /// long
        /// description
        field_nested_array:[foxglove.NestedMessage] (id: 24);
      }

      root_type ExampleMessage;
      "
    `);
  });
});
