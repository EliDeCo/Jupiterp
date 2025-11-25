<!-- 
This file is part of Jupiterp. For terms of use, please see the file
called LICENSE at the top level of the Jupiterp source tree (online at
https://github.com/atcupps/Jupiterp/LICENSE).
Copyright (C) 2024 Andrew Cupps
-->
<script lang="ts">
    import { CurrentScheduleStore, ProfsLookupStore } from "../../../stores/CoursePlannerStores";
    import type { ScheduleSelection } from "../../../types";
    import InstructorListing from './InstructorListing.svelte';
    import { getProfRatingSection } from "../../../lib/course-planner/Professors";
        import type { Instructor } from "@jupiterp/jupiterp";
    export let schedule: ScheduleSelection[];
    export let index: number;

    let selected: boolean = false;
    let current: ScheduleSelection[] = [];
    CurrentScheduleStore.subscribe((value) => {
        current = value.selections;
    });

    let profs: Record<string, Instructor>;
    ProfsLookupStore.subscribe((lookup) => { profs = lookup });

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

    function averageProfRating(): number {
        const average = schedule.reduce((total,s) => total + getProfRatingSection(profs, s), 0)/schedule.length;
        return Math.round(average*100)/100;
    }

</script>
<button on:click={toggleDisplaySchedule} class = 'text-left'>
    <div class='px-2 my-2 bg-bgSecondaryLight dark:bg-bgSecondaryDark 
                rounded-lg border-2 border-outlineLight dark:border-outlineDark
                border-solid flex flex-col
                {selected ? 'bg-hoverLight dark:bg-hoverDark' : ''}'>
        <div class="flex items-center justify-between pt-2 leading-normal">
            <div >Schedule #{index+1}</div>
            <div>Avg. rating: {averageProfRating()}</div>
        </div>
        
        
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