<script lang="ts">
    import {signedInt} from "../../helpers";
    import ElementList from "../ElementList.svelte";

    export let c;

    let time: string = 'Other';
    function setTime(t: string) {
        time = t;
    }

    let attacks: number;
    let casts: number;
    let elements: Array<any>;
    let indices: Array<number>;
    $: {
        attacks = 0;
        casts = 0;
        elements = [];
        indices = [];
        let i = 0;
        for (let move of $c.moves) {
            if (time !== 'Other') {
                if (move.time === time) {
                    if (move.type === 'Attack') attacks++;
                    else if (move.type === 'Cast') casts++;
                    else {
                        elements.push(move.element);
                        indices.push(i);
                    }
                }
            } else {
                if (typeof move.time !== 'string') {
                    if (move.type === 'Attack') attacks++;
                    else if (move.type === 'Cast') casts++;
                    else {
                        let element = Object.assign({}, move.element);
                        element['text'] += `  \n**Time:** ${move.time['Other']}`;
                        elements.push(element);
                        indices.push(i);
                    }
                }
            }
            i += 1;
        }
    }
</script>

<div class="sheet-box uk-width-2xlarge">
    <ul uk-tab class="sheet-box-nav-small">
<!--        <li><a>Actions</a></li>-->
<!--        <li><a>Boneless Actions</a></li>-->
<!--        <li><a>Reactions</a></li>-->
<!--        <li><a>Other</a></li>-->
        <li class:uk-active={time==='Action'}><a on:click={() => time = 'Action'}>Actions</a></li>
        <li class:uk-active={time==='BonusAction'}><a on:click={() => time = 'BonusAction'}>Boneless Actions</a></li>
        <li class:uk-active={time==='Reaction'}><a on:click={() => time = 'Reaction'}>Reactions</a></li>
        <li class:uk-active={time==='Other'}><a on:click={() => time = 'Other'}>Other</a></li>
    </ul>
    <div id="move-actions">
        {#if attacks !== 0}
            <table class="uk-table uk-table-small uk-table-divider uk-text-left">
                <caption>Attacks ({$c.attacks_per_action} per action)</caption>
                <thead>
                <tr>
                    <th class="uk-width-small">Weapon</th>
                    <th class="uk-table-shrink">+Hit</th>
                    <th class="uk-table-shrink">Range</th>
                    <th class="uk-width-small">Damage</th>
                    <th class="uk-table-expand">Properties</th>
                </tr>
                </thead>
                <tbody>
                {#each $c.moves as move}
                    {#if move.type === 'Attack' && move.time === time}
                        <tr>
                            <td>{move.name}</td>
                            <td>{signedInt(move.hit)}</td>
                            <td>{move.range.Fixed} ft</td>
                            <td>{move.damage}</td>
                            <td>{move.properties.join(', ')}</td>
                        </tr>
                    {/if}
                {/each}
                </tbody>
            </table>
        {/if}
        {#if casts !== 0}
            <table class="uk-table uk-table-small uk-table-divider uk-text-left">
                <caption>Casts</caption>
                <thead>
                <tr>
                    <th class="uk-width-small">Spell</th>
                    <th class="uk-table-shrink">+Hit/DC</th>
                    <th class="uk-table-shrink">Range</th>
                    <th class="uk-table-shrink">Level</th>
                    <th class="uk-table-expand">Properties</th>
                </tr>
                </thead>
                <tbody id="table-cast_actions">
                </tbody>
            </table>
        {/if}
        <ElementList {elements} {indices} container={'moves'} />
    </div>

</div>