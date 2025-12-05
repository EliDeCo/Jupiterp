<!-- 
This file is part of Jupiterp. For terms of use, please see the file
called LICENSE at the top level of the Jupiterp source tree (online at
https://github.com/atcupps/Jupiterp/LICENSE).
Copyright (C) 2024 Andrew Cupps
-->
<script lang="ts">
    import { CurrentScheduleStore } from "../../../stores/CoursePlannerStores";
    import type { ScheduleSelection } from "../../../types";
    import type { CourseBasic, Section } from "@jupiterp/jupiterp";
    import SectionListing from "./SectionListing.svelte";
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

    function getSection(selection: ScheduleSelection): Section {
        return {
            courseCode: selection.course.courseCode,
            sectionCode: selection.section.sectionCode,
            instructors: selection.section.instructors,
            meetings: selection.section.meetings,
            openSeats: selection.section.openSeats,
            totalSeats: selection.section.totalSeats,
            waitlist: selection.section.waitlist,
            holdfile: selection.section.holdfile,
        }
    }

    function getCourseBasic(selection: ScheduleSelection): CourseBasic {
        return {
            courseCode: selection.course.courseCode,
            name: selection.course.name,
            minCredits: selection.course.minCredits,
            maxCredits: selection.course.maxCredits,
            genEds: selection.course.genEds,
            conditions: selection.course.conditions,
            description: selection.course.description,
        }
    }

</script>

<button on:click={toggleDisplaySchedule} class = 'text-left'>
    <div class='px-2 my-2 bg-bgSecondaryLight dark:bg-bgSecondaryDark 
                rounded-lg border-2 border-outlineLight dark:border-outlineDark
                border-solid flex flex-col
                {selected ? 'bg-hoverLight dark:bg-hoverDark' : ''}'>
        <div >Schedule #{index+1}</div>
        
        
        <ul class="list-disc pl-5">
            {#if selected}
                {#each schedule as section}
                    <div class="text-sm">{section.course.courseCode}</div>
                    <SectionListing courseCode={section.course.courseCode}
                        section={getSection(section)}
                                course={getCourseBasic(section)} />
                    <div class="border-t-2 border-outlineLight dark:border-outlineDark w-full"></div>
                {/each}
            {:else}
                {#each schedule as section }
                    <li class="flex items-center gap-2 text-sm leading-normal">
                        <div class="inline-flex items-center gap-2 text-gray-600 ">
                            {section.course.courseCode}-{section.section.sectionCode}
                            <!-- Compact Seat data-->
                            <div class="text-sm italic dark:text-[#8892a8]">
                                {#if section.section.totalSeats > 0}
                                    {#if section.section.openSeats > 0}
                                        {section.section.openSeats} / {section.section.totalSeats} seats available
                                    {:else}
                                        Waitlist: {section.section.waitlist}
                                        {#if section.section.holdfile != null}
                                            , Holdfile: {section.section.holdfile}
                                        {/if}
                                    {/if}
                                {/if}
                            </div>
                        </div>
                    </li>
                {/each}
            {/if}
        </ul>
    </div>
</button>