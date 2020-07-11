package io.deserializers

import io.ANTLRWrapper
import com.fasterxml.jackson.core.JsonParser
import com.fasterxml.jackson.databind.DeserializationContext
import com.fasterxml.jackson.databind.deser.std.StdDeserializer
import io.JacksonWrapper
import model.access.Expression
import model.quantities.QuantityUnit
import model.quantities.QuantityType.Time

/**
 * Deserializes [TimeUnit] from YAML.
 *
 * Use through [JacksonWrapper.readFile] or [JacksonWrapper.readString] only.
 */
object TimeUnitDeserializer : StdDeserializer<Expression<QuantityUnit<Time>>>(Expression::class.java) {
    override fun deserialize(p: JsonParser?, ctxt: DeserializationContext?): Expression<QuantityUnit<Time>> {
        return ANTLRWrapper.parseTimeUnit(p!!.readValueAs(String::class.java))
    }
}
