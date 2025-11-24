<!-- 
This file is part of Jupiterp. For terms of use, please see the file
called LICENSE at the top level of the Jupiterp source tree (online at
https://github.com/atcupps/Jupiterp/LICENSE).
Copyright (C) 2024 Andrew Cupps
-->
<script lang="ts">
    import SectionListing from "./SectionListing.svelte";
    import { formatCredits, testudoLink } from "../../../lib/course-planner/Formatting";
    import { slide } from "svelte/transition";
    import CourseCondition from "./CourseCondition.svelte";
    import { AngleRightOutline } from "flowbite-svelte-icons";
    import type { Course, Section } from "@jupiterp/jupiterp";
    import { AutoGen, AutoGenCourseStore } from "../../../stores/CoursePlannerStores";
    import type { ScheduleSelection } from "../../../types";
    import InstructorListing from '../course-search/InstructorListing.svelte';
    export let schedule: ScheduleSelection[];
    export let index: number;

    let showMoreInfo = false;

    function displaySelectedSchedule() {

    }
    

</script>

<div class='px-2 my-2 bg-bgSecondaryLight dark:bg-bgSecondaryDark 
            rounded-lg border-2 border-outlineLight dark:border-outlineDark
            border-solid flex flex-col'>
    
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
    <button class='text-sm 2xl:text-base text-left
                text-secCodesLight hover:text-[#4a5366]
                dark:text-[#8892a8] hover:text-secCodesDark
                w-full flex flex-row content-center'
            title={!showMoreInfo ? "Show more schedule details" : "Hide schedule details"}
            on:click={() => {showMoreInfo = !showMoreInfo}}>
        <div class='h-full self-center transition-transform -ml-1' 
             class:rotate-90={showMoreInfo}>
            <AngleRightOutline class='h-4 w-4' />
        </div>
        <span>
            {showMoreInfo ? "Hide details" : "Show more detail"}
        </span>
    </button>

    {#if showMoreInfo}
        <div class='text-sm 2xl:text-base py-1 font-base flex flex-col leading-tight'
                transition:slide>

            <p>Extra detail here ig</p>
        </div>
    {/if}
</div>