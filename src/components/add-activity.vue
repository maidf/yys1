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
                <el-form-item label="名称" :label-position="itemLabelPosition">
                    <el-input v-model="formLabelAlign.name" />
                </el-form-item>
                <el-form-item label="消耗" :label-position="itemLabelPosition">
                    <el-input-number v-model="formLabelAlign.consume" />
                </el-form-item>
                <el-form-item>
                    <el-button type="primary" @click="add_activity">添加</el-button>
                    <el-button>Cancel</el-button>
                </el-form-item>
            </el-form>
            <el-form v-else></el-form>
        </el-main>
    </el-container>
</template>


<script setup lang="ts">
import { ref } from "vue"
import { invoke } from "@tauri-apps/api/core"
import { ElMessage, type FormItemProps, type FormProps } from 'element-plus'

const add_activity = async () => {
    invoke("add_activity", { activity: formLabelAlign.value })
        .then((res) => {
            const msg = res as string
            ElMessage.success({ message: msg })
        })
        .catch((err) => {
            const msg = err as string
            ElMessage.error({ message: msg })
        })
}

const labelPosition = ref<FormProps['labelPosition']>('right')
const itemLabelPosition = ref<FormItemProps['labelPosition']>('right')
const formLabelAlign = ref<Form>({
    name: "",
    num: 0,
    consume: 0,
})

interface Form {
    name: string
    num: number
    consume: number
}
</script>