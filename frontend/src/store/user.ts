import type { UserAuth, UserPublic } from "@/model/user";
import { defineStore } from "pinia";
import { ref } from "vue";

export const useUserStore = defineStore("user", () => {
  const userAuth = ref<UserAuth>({
    accessToken: "",
    refreshToken: "",
  });
  const userPublic = ref<UserPublic>({
    userId: -1,
    userName: "",
    userDesc: "",
    userEmail: "",
    userAvatarUrl: "",
    userLevel: -1,
    userStatus: -1,
    userIdentity: -1,
    userCreateTime: "",
    userUpdateTime: "",
  });
  return {
    userAuth,
  };
});
