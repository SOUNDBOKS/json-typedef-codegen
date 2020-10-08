using System;
using System.Collections.Generic;
using Newtonsoft.Json;
using Newtonsoft.Json.Linq;

namespace Jtd.JtdCodegenDemo
{
    /// <summary>
    /// A proper name.
    /// 
    /// Note that this is a string, and not some object with first/given name or
    /// a last/family name. We have users across many cultures, and some of
    /// these cultures use mononyms or otherwise don't map onto these concepts.
    /// </summary>
    
    public class Name 
    {



        
        public string Value { get; set; }



        private class JsonConverter : Newtonsoft.Json.JsonConverter
        {
            public override bool CanRead => true;
            public override bool CanWrite => true;

            public override bool CanConvert(System.Type objectType)
            {
                return objectType == typeof(string);
            }

            public override object ReadJson(JsonReader reader, System.Type objectType, object existingValue, JsonSerializer serializer)
            {
                return new Name { Value = serializer.Deserialize<string>(reader) };
            }

            public override void WriteJson(JsonWriter writer, object value, JsonSerializer serializer)
            {
                serializer.Serialize(writer, ((Name) value).Value);
            }
        }



    }
}