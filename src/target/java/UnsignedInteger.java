import com.fasterxml.jackson.core.JsonGenerator;
import com.fasterxml.jackson.core.JsonParser;
import com.fasterxml.jackson.core.JsonProcessingException;
import com.fasterxml.jackson.databind.DeserializationContext;
import com.fasterxml.jackson.databind.JsonDeserializer;
import com.fasterxml.jackson.databind.JsonSerializer;
import com.fasterxml.jackson.databind.SerializerProvider;
import com.fasterxml.jackson.databind.annotation.JsonDeserialize;
import com.fasterxml.jackson.databind.annotation.JsonSerialize;

import java.io.IOException;

@JsonSerialize(using = UnsignedInteger.Serializer.class)
@JsonDeserialize(using = UnsignedInteger.Deserializer.class)
public class UnsignedInteger {
    private int value;

    public UnsignedInteger(int value) {
        this.value = value;
    }

    public int getValue() {
        return value;
    }

    public void setValue(int value) {
        this.value = value;
    }

    public static class Serializer extends JsonSerializer<UnsignedInteger> {
        @Override
        public void serialize(UnsignedInteger value, JsonGenerator gen, SerializerProvider serializers) throws IOException {
            gen.writeNumber(Integer.toUnsignedLong(value.getValue()));
        }
    }

    public static class Deserializer extends JsonDeserializer<UnsignedInteger> {
        @Override
        public UnsignedInteger deserialize(JsonParser p, DeserializationContext ctxt) throws IOException, JsonProcessingException {
            return new UnsignedInteger((int) p.getLongValue());
        }
    }
}
