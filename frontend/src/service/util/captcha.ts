import type { AppResponse } from "@/model";
import type { CaptchaImageResponse } from "@/model/util";
import { axiosBase } from "@/util/axios";

async function getCaptchaEmail(userEmail: string): Promise<AppResponse<null>> {
  const response: AppResponse<null> = await axiosBase.get(
    `/util/captcha/email/${userEmail}`,
  );
  return response;
}

async function getCaptchaImage(): Promise<AppResponse<CaptchaImageResponse>> {
  const response: AppResponse<CaptchaImageResponse> =
    await axiosBase.get(`/util/captcha/image`);
  return response;
}

async function getCaptchaPhone(userPhone: string): Promise<AppResponse<null>> {
  const response: AppResponse<null> = await axiosBase.get(
    `/util/captcha/phone/${userPhone}`,
  );
  return response;
}

export { getCaptchaEmail, getCaptchaImage, getCaptchaPhone };
