package model.modifications.effects

import Engine
import com.fasterxml.jackson.annotation.JsonProperty
import com.fasterxml.jackson.databind.annotation.JsonDeserialize
import io.deserializers.IdentifierDeserializer
import model.gameObjects.Character
import model.access.Identifier

class ModifyEffect(
    @JsonProperty("modify")
    @JsonDeserialize(using = IdentifierDeserializer::class)
    val id: Identifier<*>,
    val add: String? = null,
    val subtract: String? = null,
    val multiply: String? = null,
    @JsonProperty("divide down")
    val divide: String? = null,
    @JsonProperty("divide up")
    val divideUp: String? = null,
    val append: Array<String> = arrayOf()
): Effect {
    override fun applyEffect(e: Engine) {
        TODO("Not yet implemented")
    }
}