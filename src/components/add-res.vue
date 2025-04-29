<template>
    <el-container>
        <el-header>添加掉落物类型</el-header>
        <el-main>
            <el-form v-if="formLabelAlign" :label-position="labelPosition" label-width="auto" :model="formLabelAlign"
                style="max-width: 600px">
                <el-form-item label="排布" :label-position="itemLabelPosition">
                    <el-radio-group v-model="itemLabelPosition" aria-label="item label position">
                        <el-radio-button value="">Empty</el-radio-button>
                        <el-radio-button value="left">Left</el-radio-button>
                        <el-radio-button value="right">Right</el-radio-button>
                        <el-radio-button value="top">Top</el-radio-button>
                    </el-radio-group>
                </el-form-item>
                <el-form-item label="活动" :label-position="itemLabelPosition">
                    <el-input v-model="formLabelAlign.activity" />
                </el-form-item>
                <el-form-item label="名称" :label-position="itemLabelPosition">
                    <el-input v-model="formLabelAlign.name" />
                </el-form-item>
                <el-form-item label="数量" :label-position="itemLabelPosition">
                    <el-input-number v-model="formLabelAlign.num" />
                </el-form-item>
            </el-form>
            <el-form v-else></el-form>
        </el-main>
    </el-container>
</template>


<script setup lang="ts">
import { ref } from "vue"
import { invoke } from "@tauri-apps/api/core"
import type { FormItemProps, FormProps } from 'element-plus'

const greetMsg = ref("")
const name = ref("")

async function greet() {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    greetMsg.value = await invoke("greet", { name: name.value })
}

const labelPosition = ref<FormProps['labelPosition']>('right')
const itemLabelPosition = ref<FormItemProps['labelPosition']>('right')
const formLabelAlign = ref<Form>({
    name: "",
    num: 0,
    activity: "",
})

interface Form {
    name: string
    num: number
    activity: string
}
</script>