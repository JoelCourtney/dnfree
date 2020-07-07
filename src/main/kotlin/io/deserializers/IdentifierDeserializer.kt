package io.deserializers

import com.fasterxml.jackson.core.JsonParser
import com.fasterxml.jackson.databind.DeserializationContext
import com.fasterxml.jackson.databind.deser.std.StdDeserializer
import io.ANTLRWrapper
import model.quantities.Identifier

class IdentifierDeserializer : StdDeserializer<Identifier>(Identifier::class.java) {
    override fun deserialize(p: JsonParser?, ctxt: DeserializationContext?): Identifier {
        val s = p!!.readValueAs(String::class.java)
        return ANTLRWrapper.parseIdentifier(s)
    }
}