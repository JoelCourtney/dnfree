<script lang="ts">
    export let c;

    import {render} from "../../helpers";
</script>

<div class="sheet-box uk-width-auto">
    <h1 class="box-title">Passives</h1>
    <div class="uk-width-shrink">
        <div uk-grid class="uk-grid-small uk-flex-center">
            <div class="uk-text-right">
                <ul class="uk-list uk-list-collapse">
                    <li>Perception</li>
                    <li>Investigation</li>
                    <li>Insight</li>
                </ul>
            </div>
            <div class="uk-text-left">
                <ul class="uk-list uk-list-collapse">
                    <li>{$c.passives.perception}</li>
                    <li>{$c.passives.investigation}</li>
                    <li>{$c.passives.insight}</li>
                </ul>
            </div>
        </div>
    </div>
    {#if $c.passive_notes.length}
        <p class="uk-margin-remove-bottom">{@html $c.passive_notes.map((n) => render(n, false)).join('<br/>')}</p>
    {/if}
</div>