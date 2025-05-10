<script lang="ts" setup>
import { onMounted } from "vue";
import { useRouter } from "vue-router";

const router = useRouter();

onMounted(() => {
    document.title = "网站首页";
});
</script>

<template>
    <h1>home</h1>
</template>

<style scoped></style>
