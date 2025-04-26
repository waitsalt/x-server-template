import type { AppResponse } from "@/model";
import type { CaptchaImageResponse } from "@/model/util";
import { axiosBase } from "@/util/axios";

async function captchaEmail(userEmail: string): Promise<AppResponse<null>> {
  const response: AppResponse<null> = await axiosBase.get(
    `/util/captcha/email/${userEmail}`,
  );
  return response;
}

async function captchaImage(): Promise<AppResponse<CaptchaImageResponse>> {
  const response: AppResponse<CaptchaImageResponse> =
    await axiosBase.get(`/util/captcha/image`);
  return response;
}

async function captchaPhone(userPhone: string): Promise<AppResponse<null>> {
  const response: AppResponse<null> = await axiosBase.get(
    `/util/captcha/phone/${userPhone}`,
  );
  return response;
}

export { captchaEmail, captchaImage, captchaPhone };
