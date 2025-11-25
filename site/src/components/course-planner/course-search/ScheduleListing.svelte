<!-- 
This file is part of Jupiterp. For terms of use, please see the file
called LICENSE at the top level of the Jupiterp source tree (online at
https://github.com/atcupps/Jupiterp/LICENSE).
Copyright (C) 2024 Andrew Cupps
-->
<script lang="ts">
    import { CurrentScheduleStore } from "../../../stores/CoursePlannerStores";
    import type { ScheduleSelection } from "../../../types";
    import InstructorListing from '../course-search/InstructorListing.svelte';
    export let schedule: ScheduleSelection[];
    export let index: number;

    let selected: boolean = false;
    let current: ScheduleSelection[] = [];
    CurrentScheduleStore.subscribe((value) => {
        current = value.selections;
    });

    $: if (current === schedule) {
        selected = true;
    } else {
        selected = false;
    }

    function toggleDisplaySchedule() {
        if (current === schedule) {
            CurrentScheduleStore.update(current => ({
                ...current,
                selections: []
            })); 
        } else {
            CurrentScheduleStore.update(current => ({
                ...current,
                selections: schedule
            }));
        }
    }

</script>
<button on:click={toggleDisplaySchedule} class = 'text-left'>
    <div class='px-2 my-2 bg-bgSecondaryLight dark:bg-bgSecondaryDark 
                rounded-lg border-2 border-outlineLight dark:border-outlineDark
                border-solid flex flex-col
                {selected ? 'bg-hoverLight dark:bg-hoverDark' : ''}'>
        
        <div class='pt-2'>Schedule #{index+1}</div>
        
        <ul class="list-disc pl-5">
            {#each schedule as section }
                <li class="flex items-center gap-2 text-sm leading-normal">
                    <span class="inline-flex items-center gap-2 text-gray-600 ">
                        {section.course.courseCode}-{section.section.sectionCode}
                        <InstructorListing instructor={section.section.instructors[0]}
                                profsHover={false}
                                removeHoverSection={() => {}} />
                    </span>
                </li>
            {/each}
        </ul>
    </div>
</button>