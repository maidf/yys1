<template>
    <el-container>
        <el-header>添加掉落物类型</el-header>
        <el-main v-if="formLabelAlign">
            <el-form :label-position="labelPosition" label-width="auto" :model="formLabelAlign"
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
                    <el-select v-model="formLabelAlign.activity" placeholder="活动" style="width: 240px">
                        <el-option v-for="(item, index) in activity" :key="index" :label="item.name" :value="item.id" />
                    </el-select>
                </el-form-item>
                <el-form-item label="名称" :label-position="itemLabelPosition">
                    <el-input v-model="formLabelAlign.name" />
                </el-form-item>
            </el-form>
        </el-main>
        <el-main v-else></el-main>
    </el-container>
</template>


<script setup lang="ts">
import { ref } from "vue"
import { invoke } from "@tauri-apps/api/core"
import { ElMessage, type FormItemProps, type FormProps } from 'element-plus'
import { Activity } from "../stores/type"
import { onMounted } from "vue"

const activity = ref<Activity[]>([])
const get_activity = () => {
    invoke("get_activity")
        .then((res) => {
            activity.value = res as Activity[]
        })
        .catch((err) => {
            const msg = err as string
            ElMessage.error({ message: msg })
            activity.value = []
        })
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


onMounted(() => {
    get_activity()
})
</script>