<script setup lang="ts">
  // This starter template is using Vue 3 <script setup> SFCs
  // Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
  import { invoke } from "@tauri-apps/api"
  import { ref } from "vue"
  import Search from "./components/Search.vue"
  import Editor from "./components/Editor.vue"

  let showSearch = ref(true)
  let initName = ref("")
  let initNote = ref("")
  let error = ref("")

  const onCreate = async (noteName: string) => {
    try {
      await invoke("create_note", { name: noteName })
      initNote.value = noteName
      showSearch.value = false
    } catch (err) {
      onError(err)
    }
  };
  const onOpen = async (noteName: string) => {
    try {
      const note = await invoke<string>("read_note", { name: noteName })
      initName.value = noteName
      initNote.value = note as string
      showSearch.value = false
    } catch (err) {
      onError(err)
    }
  };
  const onShowSearch = () => {
    showSearch.value = true
    initNote.value = ""
    error.value = ""
  }
  const onError = (err: any) => {
    console.error(err)
    error.value = err.toString()
  }
</script>
<template>
  <Search
    v-if="showSearch"
    @create="onCreate"
    @open="onOpen"
    @error="onError"
    />
  <Editor
    v-if="!showSearch"
    :name="initName"
    :content="initNote"
    @search="onShowSearch"
    @error="onError"
    />
  <p class="footer">
    {{ error }}
  </p>
</template>
<style scoped>
  body {
    margin: 0;
    padding: 0;
  }

  * {
    box-sizing: border-box;
  }

  .footer {
    position:fixed;
    bottom: 0;
    margin-bottom: 10px;
    padding: 0;
  }
</style>
